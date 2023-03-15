use phonos_player::player::commands::Command;
use phonos_player::player::Player;
use rspotify::AuthCodeSpotify;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;

// We possibly could only lock the sender and receiver instead of the whole struct
pub struct PlayerConnection {
    pub sender: Sender<Command>,
    pub receiver: Receiver<Command>,
}

impl PlayerConnection {
    /// This spawns a new tokio thread for a player
    pub fn new(spotify_client: AuthCodeSpotify) -> Self {
        let (web_sender, player_receiver) = channel(32);
        let (player_sender, web_receiver) = channel(32);
        let player = Player::new(spotify_client, player_sender, player_receiver);

        task::spawn(async { player.run() });

        Self {
            sender: web_sender,
            receiver: web_receiver,
        }
    }
}
