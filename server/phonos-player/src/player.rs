use std::time::Duration;

use phonos_entity::playlist;
use rspotify::model::TrackId;
use rspotify::AuthCodeSpotify;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};

use self::commands::Command;

pub mod commands;
// pub mod player_builder;
// pub mod player_options;

// pub use command::PlayerCommand;
// // pub use player_builder::PlayerBuilder;
// pub use player_options::PlayerOptions;

// #[allow(dead_code)]
// enum ElementResult {
//     Regular,
//     NextElement,
//     PreviousElement,
//     EndPlayback,
// }

// // I think the next/prev can be combined to some "change element" thing
// // and have the playback_tick change which element is playing
// enum TickResult {
//     Regular,
//     Paused,
//     NextElement,
//     PreviousElement,
//     EndPlayback,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlaybackState {
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
    current_song_id: Option<TrackId<'static>>,
    current_song_position: Option<Duration>,
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
    sender: Sender<Command>,
    receiver: Receiver<Command>,
    state: Option<PlaybackState>,
}

impl Player {
    pub fn new(
        spotify_client: AuthCodeSpotify,
        sender: Sender<Command>,
        receiver: Receiver<Command>,
    ) -> Self {
        Self {
            spotify_client,
            sender,
            receiver,
            state: None,
        }
    }

    pub fn run(mut self) {
        loop {
            if let Ok(command) = self.receiver.try_recv() {
                match command {
                    Command::Play {
                        playlist,
                        element_index,
                        song_index,
                    } => {
                        println!("Beginning playback of: ");
                        unimplemented!()
                    }
                    _ => {
                        println!("Unimplemented command");
                        unimplemented!()
                    }
                }
            }
        }
    }
}

// impl Player {
//     pub fn new(spotify_client: AuthCodeSpotify, playlist: Playlist, device_id: String) -> Self {
//         Self {
//             spotify_client,
//             command_receiver: None,
//             playlist,
//             device_id,
//             state: None,
//         }
//     }

//     pub fn save_state(&self) {
//         unimplemented!()
//     }

//     pub fn load_state(&self) {
//         unimplemented!()
//     }

//     pub async fn play(mut self, player_options: PlayerOptions) -> Sender<PlayerCommand> {
//         if self.state.is_none() {
//             let order = generate_order(self.playlist.elements.len(), &player_options);

//             self.state = Some(PlaybackState {
//                 order,
//                 current_element: 0,
//                 current_song: 0,
//                 current_song_id: None,
//                 current_song_position: Duration::ZERO,
//             });
//         }

//         let (sender, receiver) = channel::<PlayerCommand>();

//         self.command_receiver = Some(receiver);

//         task::spawn(async move {
//             loop {
//                 let index = self.state.as_ref().unwrap().get_current_element_index();
//                 let element = &self.playlist.elements[index];
//                 println!("Playing element: {}", element.name());

//                 let element_result = self.play_element(index).await;
//                 match element_result {
//                     ElementResult::Regular | ElementResult::NextElement => {
//                         self.state.as_mut().unwrap().increment_playback();
//                     }

//                     ElementResult::PreviousElement => {
//                         self.state.as_mut().unwrap().decrement_playback();
//                     }

//                     ElementResult::EndPlayback => {
//                         println!("Ending playback");
//                         return;
//                     }
//                 }
//             }
//         });

//         sender
//     }

//     async fn play_element(&mut self, element_index: usize) -> ElementResult {
//         let element = &self.playlist.elements[element_index];

//         // TODO: Change playback based on if it's a songlist or a spotifyplaylist
//         let uris = element.to_playable_ids();

//         // Begin playback of entire element
//         self.spotify_client
//             .start_uris_playback(uris, Some(&self.device_id), Some(Offset::Position(0)), None)
//             .await
//             .unwrap();

//         // Disable repeat
//         self.spotify_client
//             .repeat(RepeatState::Off, Some(&self.device_id))
//             .await
//             .unwrap();

//         // Disable shuffle
//         self.spotify_client
//             .shuffle(false, Some(&self.device_id))
//             .await
//             .unwrap();

//         let mut pause_start: Option<Instant> = None;

//         loop {
//             let tick_result = self.playback_tick(element_index).await;
//             // TODO: Save new state
//             match tick_result {
//                 TickResult::NextElement => return ElementResult::NextElement,
//                 TickResult::PreviousElement => return ElementResult::PreviousElement,
//                 TickResult::EndPlayback => return ElementResult::EndPlayback,
//                 TickResult::Paused => {
//                     if let Some(start) = pause_start {
//                         if start.elapsed().as_secs() >= 300 {
//                             // return ElementResult::EndPlayback;
//                         }
//                     } else {
//                         pause_start = Some(Instant::now())
//                     }
//                 }
//                 _ => {}
//             }

//             tokio::time::sleep(time::Duration::from_secs(1)).await;
//         }

//         // ElementResult::Regular
//     }

//     async fn playback_tick(&mut self, element_index: usize) -> TickResult {
//         let element = &self.playlist.elements[element_index];
//         if let Ok(Some(playback)) = self
//             .spotify_client
//             .current_playback(None, None::<Vec<_>>)
//             .await
//         {
//             if let Some(PlayableItem::Track(track)) = playback.item {
//                 if let Some(ref current_song_id) = track.id {
//                     let prog = playback.progress.unwrap();

//                     // TODO: Change playback based on if it's a songlist or a spotifyplaylist
//                     let song_index = element
//                         .songs
//                         .iter()
//                         .position(|e| e.id == *current_song_id)
//                         .or(Some(0));

//                     if let Some(song_index) = song_index {
//                         if song_index == 0 && !playback.is_playing && prog == Duration::ZERO {
//                             return TickResult::NextElement;
//                         }

//                         if Some(current_song_id)
//                             != self.state.as_ref().unwrap().current_song_id.as_ref()
//                         {
//                             println!("\tPlaying song: {}", track.name);
//                             self.state.as_mut().unwrap().current_song = song_index;
//                             self.state.as_mut().unwrap().current_song_id =
//                                 Some(current_song_id.clone());
//                         }

//                         if !playback.is_playing {
//                             return TickResult::Paused;
//                         }
//                     } else {
//                         return TickResult::EndPlayback;
//                     }
//                 }
//             }
//         }

//         if let Some(receiver) = &self.command_receiver {
//             if let Ok(cmd) = receiver.try_recv() {
//                 match cmd {
//                     PlayerCommand::NextElement => {
//                         println!("Going to next element");
//                         return TickResult::NextElement;
//                     }
//                     PlayerCommand::PrevElement => {
//                         println!("Going to previous element");
//                         return TickResult::PreviousElement;
//                     }
//                 }
//             }
//         }

//         TickResult::Regular
//     }
// }

// fn generate_order(len: usize, options: &PlayerOptions) -> Vec<usize> {
//     let mut nums: Vec<usize> = (0..len).collect();

//     if options.shuffle {
//         if let Some(start_index) = options.start_element {
//             nums.remove(start_index);
//             nums.shuffle(&mut thread_rng());
//             nums.insert(0, start_index);
//         } else {
//             nums.shuffle(&mut thread_rng());
//         }
//     } else {
//         let start_index = options.start_element.unwrap_or(0);
//         nums = (start_index..len).chain(0..start_index).collect();
//     }

//     nums
// }
