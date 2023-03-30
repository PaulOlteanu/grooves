use anyhow::{anyhow, Context};
use chrono::Duration;
use grooves_entity::playlist::{self, PlaylistElement, Song};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rspotify::model::{PlayableItem, RepeatState};
use rspotify::prelude::{BaseClient, OAuthClient};
use rspotify::{AuthCodeSpotify, ClientResult};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};

use self::commands::{Command, PlayData};

pub mod commands;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlaybackStatus {
    Paused,
    Playing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaybackInfo {
    image_url: String,
    song_name: String,
    album_name: String,
    artist_name: String,
    playback_status: PlaybackStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlayerState {
    device_id: Option<String>,
    playlist: playlist::Model,

    /// A list of indices into playlist elements
    /// if order = [2, 0, 1], that corresponds to playing the 2nd then 0th then 1st elements in the playlist
    order: Vec<usize>,

    /// An index into order
    /// if order = [2, 0, 1] and current = 1, we're currently playing the 0th element in the playlist
    current_element: usize,

    /// The index of the current song in the element
    current_song: usize,
}

impl PlayerState {
    fn get_current_element(&self) -> &PlaylistElement {
        let index = self.order[self.current_element];
        &self.playlist.elements[index]
    }

    fn get_current_song(&self) -> &Song {
        let index = self.order[self.current_element];
        let element = &self.playlist.elements[index];
        &element.songs[self.current_song]
    }

    fn increment_current(&mut self) {
        self.current_element = (self.current_element + 1) % self.order.len();
        self.current_song = 0;
    }

    fn decrement_current(&mut self) {
        if self.current_element == 0 {
            self.current_element = self.order.len() - 1;
        } else {
            self.current_element -= 1;
        }
        self.current_song = 0;
    }

    async fn get_playback_info(&self, client: &AuthCodeSpotify) -> PlaybackInfo {
        let element = self.get_current_element();
        let song = &element.songs[self.current_song];

        let full_song = client.track(song.spotify_id.clone()).await.unwrap();
        let image = full_song.album.images.iter().max_by_key(|i| i.width);
        let image_url = if let Some(image) = image {
            image.url.clone()
        } else {
            "".to_string()
        };

        let artist = full_song.artists.iter().map(|a| &a.name).join(", ");

        PlaybackInfo {
            image_url,
            song_name: song.name.clone(),
            album_name: element.name.clone(),
            artist_name: artist,
            playback_status: PlaybackStatus::Paused,
        }
    }
}

pub struct Player {
    spotify_client: AuthCodeSpotify,
    sender: watch::Sender<Option<PlaybackInfo>>,
    receiver: mpsc::UnboundedReceiver<Command>,
    playback_state: PlayerState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum TickResult {
    Changed,
    Unchanged,
}

// TODO: Clean this up
impl Player {
    pub async fn new(
        spotify_client: AuthCodeSpotify,
        sender: watch::Sender<Option<PlaybackInfo>>,
        receiver: mpsc::UnboundedReceiver<Command>,
        play_data: PlayData,
    ) -> Self {
        let new_state = PlayerState {
            device_id: None,
            order: generate_order(play_data.playlist.elements.0.len(), play_data.element_index),
            playlist: play_data.playlist,
            current_element: 0,
            current_song: 0,
        };

        let element = new_state.get_current_element();

        let res = play_element(&spotify_client, element).await;

        if res.is_ok() {
            let _ = sender.send(Some(new_state.get_playback_info(&spotify_client).await));
        }

        Self {
            spotify_client,
            sender,
            receiver,
            playback_state: new_state,
        }
    }

    pub async fn run(mut self) {
        let mut failures = 0;

        loop {
            if let Ok(command) = self.receiver.try_recv() {
                match command {
                    Command::Play(PlayData {
                        playlist,
                        element_index,
                        ..
                    }) => {
                        println!("Playing playlist: {:?}", playlist.name);

                        let new_state = PlayerState {
                            device_id: None,
                            order: generate_order(playlist.elements.0.len(), element_index),
                            playlist,
                            current_element: 0,
                            current_song: 0,
                        };

                        self.playback_state = new_state;
                        let element = self.playback_state.get_current_element();

                        let res = play_element(&self.spotify_client, element).await;

                        if res.is_ok() {
                            if self
                                .sender
                                .send(Some(
                                    self.playback_state
                                        .get_playback_info(&self.spotify_client)
                                        .await,
                                ))
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                    Command::NextElement => {
                        self.playback_state.increment_current();

                        let element = self.playback_state.get_current_element();
                        let res = play_element(&self.spotify_client, element).await;

                        if res.is_ok() {
                            if self
                                .sender
                                .send(Some(
                                    self.playback_state
                                        .get_playback_info(&self.spotify_client)
                                        .await,
                                ))
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                    Command::PrevElement => {
                        self.playback_state.decrement_current();

                        let element = self.playback_state.get_current_element();
                        let res = play_element(&self.spotify_client, element).await;

                        if res.is_ok() {
                            if self
                                .sender
                                .send(Some(
                                    self.playback_state
                                        .get_playback_info(&self.spotify_client)
                                        .await,
                                ))
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                    cmd => {
                        println!("Unimplemented command");
                        println!("{:?}", cmd)
                    }
                }
            }

            let tick_result = self.tick().await;

            if let Ok(TickResult::Changed) = tick_result {
                if self
                    .sender
                    .send(Some(
                        self.playback_state
                            .get_playback_info(&self.spotify_client)
                            .await,
                    ))
                    .is_err()
                {
                    return;
                }
            }

            if tick_result.is_err() {
                println!("Tick errored: {:?}", tick_result);
                failures += 1;
            } else {
                failures = 0;
            };

            if failures >= 5 {
                println!("Player exiting");
                return;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    async fn tick(&mut self) -> Result<TickResult, anyhow::Error> {
        // Get spotify playback state
        let playback = self
            .spotify_client
            .current_playback(None, None::<Vec<_>>)
            .await?
            .context("no current playback")?;

        let current_element = self.playback_state.get_current_element();

        if !playback.is_playing {
            if let Some(prog) = playback.progress {
                if let Some(PlayableItem::Track(song)) = playback.item {
                    if let Some(id) = song.id {
                        if current_element.songs[0].spotify_id == id && prog == Duration::zero() {
                            // Play next element and return
                            self.playback_state.increment_current();
                            let element = self.playback_state.get_current_element();
                            play_element(&self.spotify_client, element).await?;

                            return Ok(TickResult::Changed);
                        }
                    }
                }
            }

            // TODO: begin pause watching
            return Err(anyhow!("paused"));
        }

        let playing_item = if let Some(item) = playback.item {
            item
        } else {
            // This shouldn't happen if is_playing is true
            println!("No playing item even though playback.is_playing is true");
            println!("Playback state:");
            println!("{:?}", self.playback_state);
            return Err(anyhow!("no playing item despite is_playing being true"));
        };

        let playing_song = if let PlayableItem::Track(song) = playing_item {
            song
        } else {
            // A podcast episode is playing
            // TODO: Handle this differently
            return Err(anyhow!("unexpected item playing"));
        };

        let playing_id = if let Some(id) = playing_song.id {
            id
        } else {
            // It's playing a local file
            // TODO: Handle this differently
            return Err(anyhow!("unexpected item playing"));
        };

        let playing_index = self
            .playback_state
            .get_current_element()
            .songs
            .iter()
            .position(|s| s.spotify_id == playing_id);

        if let Some(idx) = playing_index {
            if idx != self.playback_state.current_song {
                self.playback_state.current_song = idx;

                return Ok(TickResult::Changed);
            }
        } else {
            // The current element doesn't contain the currently playing song
            // TODO: Handle this differently
            return Err(anyhow!("unexpected item playing"));
        }

        Ok(TickResult::Unchanged)
    }

    async fn play_playlist(&mut self, play_data: PlayData) -> Result<(), ()> {
        todo!()
    }
}

async fn play_element(
    spotify_client: &AuthCodeSpotify,
    element: &PlaylistElement,
) -> ClientResult<()> {
    // Disable repeat
    spotify_client.repeat(RepeatState::Off, None).await?;

    // Disable shuffle
    spotify_client.shuffle(false, None).await?;

    let song_ids = element.songs.iter().map(|s| s.spotify_id.clone().into());
    spotify_client
        .start_uris_playback(song_ids, None, None, None)
        .await
}

fn generate_order(len: usize, start_index: Option<usize>) -> Vec<usize> {
    let mut nums: Vec<usize> = (0..len).collect();

    if let Some(start_index) = start_index {
        nums.remove(start_index);
        nums.shuffle(&mut thread_rng());
        nums.insert(0, start_index);
    } else {
        nums.shuffle(&mut thread_rng());
    }

    nums
}
