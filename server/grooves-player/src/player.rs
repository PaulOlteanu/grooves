use grooves_entity::playlist::{self, PlaylistElement};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientResult};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};
use tokio::task::yield_now;

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
    // pub fn new(playlist: playlist::Model,) -> Self {
    //     Self {
    //         device_id: None,
    //         playlist,
    //         order: generate_order(playlist.elements.len, options)
    //     }
    // }
}

// impl PlaybackState {
//     pub fn get_current_element_index(&self) -> usize {
//         self.order[self.current_element]
//     }

//     pub fn increment_playback(&mut self) {
//         self.current_element = (self.current_element + 1) % self.order.len();
//     }

//     pub fn decrement_playback(&mut self) {
//         if self.current_element == 0 {
//             self.current_element = self.order.len() - 1;
//         } else {
//             self.current_element -= 1;
//         }
//     }
// }

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
                        let elements = &*self.playback_state.as_ref().unwrap().playlist.elements;
                        let element = &elements[self.playback_state.as_ref().unwrap().order[0]];

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

            // Tick playback
            self.tick();
            yield_now().await;
        }
    }

    fn tick(&mut self) {}
}

async fn play_element(
    spotify_client: &AuthCodeSpotify,
    element: &PlaylistElement,
) -> ClientResult<()> {
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
