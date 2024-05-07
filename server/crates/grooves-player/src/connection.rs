use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};

use futures::Future;
use rspotify::AuthCodeSpotify;
use tokio::sync::{mpsc, watch};
use tokio::task;

use crate::player::commands::Command;
use crate::player::{PlaybackInfo, Player};

pub struct FutureConnectionData {
    pub connection: Option<PlayerConnection>,
    pub waker: Option<Waker>,
}

pub struct FutureConnection {
    data: Arc<Mutex<FutureConnectionData>>,
}

impl FutureConnection {
    pub fn new(data: Arc<Mutex<FutureConnectionData>>) -> Self {
        Self { data }
    }
}

impl Future for FutureConnection {
    type Output = PlayerConnection;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut data = self.data.lock().unwrap();
        if let Some(conn) = data.connection.clone() {
            Poll::Ready(conn)
        } else {
            data.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// We possibly could only lock the sender and receiver instead of the whole struct
#[derive(Clone)]
pub struct PlayerConnection {
    pub sender: mpsc::UnboundedSender<Command>,
    pub receiver: watch::Receiver<Option<PlaybackInfo>>,
}

impl PlayerConnection {
    /// This spawns a new tokio thread for a player
    pub fn new(spotify_client: AuthCodeSpotify) -> Self {
        println!("Creating new player");
        let (manager_sender, player_receiver) = mpsc::unbounded_channel();
        let (player_sender, manager_receiver) = watch::channel(None);
        let player = Player::new(spotify_client, player_sender, player_receiver);

        task::spawn(async move {
            let _ = player.run().await;
        });

        Self {
            sender: manager_sender,
            receiver: manager_receiver,
        }
    }
}
