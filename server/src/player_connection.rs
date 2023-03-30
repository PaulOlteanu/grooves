use grooves_player::player::commands::{Command, PlayData};
use grooves_player::player::{PlaybackInfo, Player};
use rspotify::AuthCodeSpotify;
use tokio::sync::{mpsc, watch};
use tokio::task;

// We possibly could only lock the sender and receiver instead of the whole struct
#[derive(Clone)]
pub struct PlayerConnection {
    pub sender: mpsc::UnboundedSender<Command>,
    pub receiver: watch::Receiver<Option<PlaybackInfo>>,
}

impl PlayerConnection {
    /// This spawns a new tokio thread for a player
    pub async fn new(spotify_client: AuthCodeSpotify, play_data: PlayData) -> Self {
        println!("Creating new player");
        let (web_sender, player_receiver) = mpsc::unbounded_channel();
        let (player_sender, web_receiver) = watch::channel(None);
        let player = Player::new(spotify_client, player_sender, player_receiver, play_data).await;

        task::spawn(async move { player.run().await });

        Self {
            sender: web_sender,
            receiver: web_receiver,
        }
    }
}
