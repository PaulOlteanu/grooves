use anyhow::anyhow;
use chrono::Duration;
use grooves_entity::playlist::{self, Entity as Playlist, PlaylistElement, Song};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rspotify::model::{FullTrack, PlayableItem, RepeatState};
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientResult};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};

use self::commands::Command;

pub mod commands;
pub mod error;
use error::PlayerError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaybackInfo {
    image_url: String,
    song_name: String,
    album_name: String,
    artists: String,
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

    fn get_playback_info(&self) -> PlaybackInfo {
        let element = self.get_current_element();
        let song = self.get_current_song();

        PlaybackInfo {
            image_url: song.image_url.clone(),
            song_name: song.name.clone(),
            album_name: element.name.clone(),
            artists: song.artists.clone(),
        }
    }
}

pub struct Player {
    spotify_client: AuthCodeSpotify,
    sender: watch::Sender<Option<PlaybackInfo>>,
    receiver: mpsc::UnboundedReceiver<Command>,
    playback_state: Option<PlayerState>,
    db: DatabaseConnection,
}

enum TickResult {
    Changed,
    Unchanged,
}

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

    pub async fn run(mut self) -> Result<(), PlayerError> {
        let mut failures = 0;

        loop {
            if let Ok(command) = self.receiver.try_recv() {
                self.handle_command(command).await?;
            }

            if self.playback_state.is_some() {
                let tick_result = self.tick().await;

                if let Ok(result) = tick_result {
                    failures = 0;

                    if let TickResult::Changed = result {
                        self.send_state().await?
                    }
                } else if let Err(e) = tick_result {
                    println!("Tick errored: {:?}", e);
                    failures += 1;
                }
            };

            if failures >= 5 {
                println!("Player exiting");
                return Err(PlayerError::TooManyErrors);
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    async fn tick(&mut self) -> Result<TickResult, anyhow::Error> {
        let Some(playback) = self.spotify_client.current_playback(None, None::<Vec<_>>).await? else {
            return Ok(TickResult::Unchanged);
        };

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

                            return Ok(TickResult::Changed);
                        }
                    }
                }
            }
        }

        let Some(PlayableItem::Track(FullTrack{id: Some(playing_id), ..})) = playback.item else {
            return Err(anyhow!("couldn't get current playback id"));
        };

        let playing_index = playback_state
            .get_current_element()
            .songs
            .iter()
            .position(|s| s.spotify_id == playing_id);

        if let Some(idx) = playing_index {
            if idx != playback_state.current_song {
                playback_state.current_song = idx;

                return Ok(TickResult::Changed);
            }
        } else {
            // The current element doesn't contain the currently playing song
            return Err(anyhow!("unexpected item playing"));
        }

        Ok(TickResult::Unchanged)
    }

    async fn handle_command(&mut self, command: Command) -> Result<(), PlayerError> {
        if let Command::Play {
            playlist_id,
            element_index,
            ..
        } = command
        {
            let Ok(Some(playlist)) =
                    Playlist::find_by_id(playlist_id).one(&self.db).await
                else {
                    return Err(PlayerError::CommandError);
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
                return Err(PlayerError::ChannelError);
            }
            return Ok(());
        }

        let Some(playback_state) = self.playback_state.as_mut() else {
            return Err(PlayerError::NoPlayback)
        };

        match command {
            Command::Play { .. } => unreachable!(),
            Command::Pause => self.spotify_client.pause_playback(None).await?,
            Command::Resume => self.spotify_client.resume_playback(None, None).await?,
            Command::NextSong => self.spotify_client.next_track(None).await?,
            Command::PrevSong => self.spotify_client.previous_track(None).await?,

            Command::NextElement => {
                playback_state.increment_current();

                let element = playback_state.get_current_element();
                let res = play_element(&self.spotify_client, element).await;

                if res.is_ok() && self.send_state().await.is_err() {
                    return Err(PlayerError::ChannelError);
                }
            }

            Command::PrevElement => {
                playback_state.decrement_current();

                let element = playback_state.get_current_element();
                let res = play_element(&self.spotify_client, element).await;

                if res.is_ok() {
                    self.send_state().await?
                }
            }

            Command::AddToQueue | Command::RemoveFromQueue | Command::Exit => {
                println!("Unimplemented command");
            }
        }

        Ok(())
    }

    async fn send_state(&self) -> Result<(), PlayerError> {
        if let Some(playback_state) = &self.playback_state {
            let playback_info = playback_state.get_playback_info();
            if self.sender.send(Some(playback_info)).is_ok() {
                return Ok(());
            }
        }

        Err(PlayerError::NoPlayback)
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
