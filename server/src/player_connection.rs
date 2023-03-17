use grooves_player::player::commands::Command;
use grooves_player::player::{PlaybackState, Player};
use rspotify::AuthCodeSpotify;
use tokio::sync::{mpsc, watch};
use tokio::task;

// We possibly could only lock the sender and receiver instead of the whole struct
pub struct PlayerConnection {
    pub sender: mpsc::UnboundedSender<Command>,
    pub receiver: watch::Receiver<Option<PlaybackState>>,
}

impl PlayerConnection {
    /// This spawns a new tokio thread for a player
    pub fn new(spotify_client: AuthCodeSpotify) -> Self {
        let (web_sender, player_receiver) = mpsc::unbounded_channel();
        let (player_sender, web_receiver) = watch::channel(None);
        let player = Player::new(spotify_client, player_sender, player_receiver);

        task::spawn(async move { player.run().await });

        Self {
            sender: web_sender,
            receiver: web_receiver,
        }
    }
}
