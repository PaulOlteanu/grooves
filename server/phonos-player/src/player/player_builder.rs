// use std::sync::mpsc::Sender;

// use rspotify::AuthCodeSpotify;

// use super::{Player, PlayerCommand, PlayerOptions};

// pub struct PlayerBuilder {
//     spotify_client: Option<AuthCodeSpotify>,
//     playlist: Option<Playlist>,
//     device_id: Option<String>,
//     player_options: PlayerOptions,
// }

// impl PlayerBuilder {
//     pub fn new() -> Self {
//         Self {
//             spotify_client: None,
//             playlist: None,
//             device_id: None,
//             player_options: PlayerOptions::new(),
//         }
//     }

//     pub fn spotify_client(mut self, client: AuthCodeSpotify) -> Self {
//         self.spotify_client = Some(client);
//         self
//     }

//     pub fn playlist(mut self, playlist: Playlist) -> Self {
//         self.playlist = Some(playlist);
//         self
//     }

//     pub fn device_id(mut self, device_id: String) -> Self {
//         self.device_id = Some(device_id);
//         self
//     }

//     pub fn shuffle(mut self, val: bool) -> Self {
//         self.player_options.shuffle = val;
//         self
//     }

//     pub fn start_element(mut self, start_element: usize) -> Self {
//         self.player_options.start_element = Some(start_element);
//         self
//     }

//     pub async fn play(self) -> Option<Sender<PlayerCommand>> {
//         let client = self.spotify_client?;
//         let playlist = self.playlist?;
//         let device_id = self.device_id?;

//         let player = Player::new(client, playlist, device_id);

//         Some(player.play(self.player_options).await)
//     }
// }
