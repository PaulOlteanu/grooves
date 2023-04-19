use anyhow::{anyhow, Context};
use chrono::Duration;
use grooves_entity::playlist::{self, Entity as Playlist, PlaylistElement, Song};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rspotify::model::{PlayableItem, RepeatState};
use rspotify::prelude::{BaseClient, OAuthClient};
use rspotify::{AuthCodeSpotify, ClientResult};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};

use self::commands::Command;

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

    async fn get_playback_info(
        &self,
        client: &AuthCodeSpotify,
    ) -> Result<PlaybackInfo, anyhow::Error> {
        let element = self.get_current_element();
        let song = &element.songs[self.current_song];

        let full_song = client.track(song.spotify_id.clone()).await?;
        let image = full_song.album.images.iter().max_by_key(|i| i.width);
        let image_url = if let Some(image) = image {
            image.url.clone()
        } else {
            "".to_string()
        };

        let artist = full_song.artists.iter().map(|a| &a.name).join(", ");

        Ok(PlaybackInfo {
            image_url,
            song_name: song.name.clone(),
            album_name: element.name.clone(),
            artist_name: artist,
            playback_status: PlaybackStatus::Paused,
        })
    }
}

pub struct Player {
    spotify_client: AuthCodeSpotify,
    sender: watch::Sender<Option<PlaybackInfo>>,
    receiver: mpsc::UnboundedReceiver<Command>,
    playback_state: Option<PlayerState>,
    db: DatabaseConnection,
}

// TODO: Clean this up
impl Player {
    pub fn new(
        spotify_client: AuthCodeSpotify,
        db: DatabaseConnection,
        sender: watch::Sender<Option<PlaybackInfo>>,
        receiver: mpsc::UnboundedReceiver<Command>,
    ) -> Self {
        Self {
            spotify_client,
            db,
            sender,
            receiver,
            playback_state: None,
        }
    }

    pub async fn run(mut self) {
        let mut failures = 0;

        loop {
            if let Ok(command) = self.receiver.try_recv() {
                match command {
                    Command::Play {
                        playlist_id,
                        element_index,
                        ..
                    } => {
                        let playlist = if let Ok(Some(playlist)) =
                            Playlist::find_by_id(playlist_id).one(&self.db).await
                        {
                            playlist
                        } else {
                            return;
                        };

                        let new_state = PlayerState {
                            device_id: None,
                            order: generate_order(playlist.elements.0.len(), element_index),
                            playlist,
                            current_element: 0,
                            current_song: 0,
                        };

                        self.playback_state = Some(new_state);
                        let playback_state = self.playback_state.as_ref().unwrap();
                        let element = playback_state.get_current_element();

                        let res = play_element(&self.spotify_client, element).await;

                        if res.is_ok() && self.send_state().await.is_err() {
                            return;
                        }
                    }

                    Command::NextElement => {
                        if let Some(playback_state) = self.playback_state.as_mut() {
                            playback_state.increment_current();

                            let element = playback_state.get_current_element();
                            let res = play_element(&self.spotify_client, element).await;

                            if res.is_ok() && self.send_state().await.is_err() {
                                return;
                            }
                        } else {
                            return;
                        }
                    }

                    Command::PrevElement => {
                        if let Some(playback_state) = self.playback_state.as_mut() {
                            playback_state.decrement_current();

                            let element = playback_state.get_current_element();
                            let res = play_element(&self.spotify_client, element).await;

                            if res.is_ok() && self.send_state().await.is_err() {
                                return;
                            }
                        } else {
                            return;
                        }
                    }

                    cmd => {
                        println!("Unimplemented command");
                        println!("{:?}", cmd)
                    }
                }
            }

            if self.playback_state.is_some() {
                if let Err(e) = self.tick().await {
                    println!("Tick errored: {:?}", e);
                    failures += 1;
                } else {
                    failures = 0;
                };
            };

            if self.send_state().await.is_err() {
                return;
            }

            if failures >= 5 {
                println!("Player exiting");
                return;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    async fn tick(&mut self) -> Result<(), anyhow::Error> {
        // Get spotify playback state
        let playback = self
            .spotify_client
            .current_playback(None, None::<Vec<_>>)
            .await?
            .context("no current playback")?;

        let mut playback_state = self.playback_state.as_mut().unwrap();
        let current_element = playback_state.get_current_element();

        // If the playback stopped at the first song of the element, with 0 progress, we need to play the next element
        if !playback.is_playing {
            if let Some(prog) = playback.progress {
                if let Some(PlayableItem::Track(song)) = &playback.item {
                    if let Some(id) = &song.id {
                        if current_element.songs[0].spotify_id == *id && prog == Duration::zero() {
                            playback_state.increment_current();
                            let element = playback_state.get_current_element();
                            play_element(&self.spotify_client, element).await?;

                            return Ok(());
                        }
                    }
                }
            }
        }

        let playing_item = if let Some(item) = playback.item {
            item
        } else {
            return Err(anyhow!("no playing item despite is_playing being true"));
        };

        let playing_song = if let PlayableItem::Track(song) = playing_item {
            song
        } else {
            // A podcast episode is playing
            return Err(anyhow!("unexpected item playing"));
        };

        let playing_id = if let Some(id) = playing_song.id {
            id
        } else {
            // It's playing a local file
            return Err(anyhow!("unexpected item playing"));
        };

        let playing_index = playback_state
            .get_current_element()
            .songs
            .iter()
            .position(|s| s.spotify_id == playing_id);

        if let Some(idx) = playing_index {
            if idx != playback_state.current_song {
                playback_state.current_song = idx;

                return Ok(());
            }
        } else {
            // The current element doesn't contain the currently playing song
            return Err(anyhow!("unexpected item playing"));
        }

        Ok(())
    }

    async fn send_state(&self) -> Result<(), ()> {
        if let Some(playback_state) = &self.playback_state {
            if let Ok(playback_info) = playback_state.get_playback_info(&self.spotify_client).await
            {
                if self.sender.send(Some(playback_info)).is_ok() {
                    return Ok(());
                }
            }
        }

        Err(())
    }
}

async fn play_element(
    spotify_client: &AuthCodeSpotify,
    element: &PlaylistElement,
) -> ClientResult<()> {
    spotify_client.repeat(RepeatState::Off, None).await?;
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
