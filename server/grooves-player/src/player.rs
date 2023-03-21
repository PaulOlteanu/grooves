use anyhow::{anyhow, Context};
use chrono::Duration;
use grooves_entity::playlist::{self, PlaylistElement};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rspotify::model::{PlayableItem, RepeatState};
use rspotify::prelude::{Id, OAuthClient};
use rspotify::{AuthCodeSpotify, ClientResult};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};

use self::commands::Command;

pub mod commands;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaybackState {
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

impl PlaybackState {
    pub fn get_current_element(&self) -> &PlaylistElement {
        let index = self.order[self.current_element];
        &self.playlist.elements[index]
    }

    // pub fn new(playlist: playlist::Model,) -> Self {
    //     Self {
    //         device_id: None,
    //         playlist,
    //         order: generate_order(playlist.elements.len, options)
    //     }
    // }

    pub fn increment_current(&mut self) {
        self.current_element = (self.current_element + 1) % self.order.len();
    }

    pub fn decrement_current(&mut self) {
        if self.current_element == 0 {
            self.current_element = self.order.len() - 1;
        } else {
            self.current_element -= 1;
        }
    }
}

pub struct Player {
    spotify_client: AuthCodeSpotify,
    sender: watch::Sender<Option<PlaybackState>>,
    receiver: mpsc::UnboundedReceiver<Command>,
    playback_state: Option<PlaybackState>,
}

impl Player {
    pub fn new(
        spotify_client: AuthCodeSpotify,
        sender: watch::Sender<Option<PlaybackState>>,
        receiver: mpsc::UnboundedReceiver<Command>,
    ) -> Self {
        Self {
            spotify_client,
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
                        playlist,
                        element_index,
                        ..
                    } => {
                        println!("Playing playlist: {:?}", playlist.name);

                        let new_state = PlaybackState {
                            device_id: None,
                            order: generate_order(playlist.elements.0.len(), element_index),
                            playlist,
                            current_element: 0,
                            current_song: 0,
                        };

                        self.playback_state = Some(new_state);
                        let element = self.playback_state.as_ref().unwrap().get_current_element();

                        let res = play_element(&self.spotify_client, element).await;

                        if res.is_ok() {
                            self.sender.send(self.playback_state.clone());
                        }
                    }
                    _ => {
                        println!("Unimplemented command");
                        unimplemented!()
                    }
                }
            }

            if self.playback_state.is_some() {
                // Tick playback

                let tick_result = self.tick().await;

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

        let playback_state = if let Some(pb) = &mut self.playback_state {
            pb
        } else {
            return Err(anyhow!("no playback state"));
        };

        let current_element = playback_state.get_current_element();

        if !playback.is_playing {
            if let Some(prog) = playback.progress {
                if let Some(PlayableItem::Track(song)) = playback.item {
                    if let Some(id) = song.id {
                        if current_element.songs[0].spotify_id == id && prog == Duration::zero() {
                            // Play next element and return
                            playback_state.increment_current();

                            let element = playback_state.get_current_element();

                            return Ok(play_element(&self.spotify_client, element).await?);
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
            println!("{:?}", playback_state);
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

        if !current_element
            .songs
            .iter()
            .map(|s| &s.spotify_id)
            .contains(&playing_id)
        {
            // The current element doesn't contain the currently playing song
            // TODO: Handle this differently
            return Err(anyhow!("unexpected item playing"));
        }

        Ok(())
    }
}

async fn play_element(
    spotify_client: &AuthCodeSpotify,
    element: &PlaylistElement,
) -> ClientResult<()> {
    let song_ids = element.songs.iter().map(|s| s.spotify_id.clone().into());
    spotify_client
        .start_uris_playback(song_ids, None, None, None)
        .await?;

    // Disable repeat
    spotify_client.repeat(RepeatState::Off, None).await?;

    // Disable shuffle
    spotify_client.shuffle(false, None).await?;

    Ok(())
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
