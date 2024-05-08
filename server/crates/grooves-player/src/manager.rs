use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use grooves_model::User;
use rspotify::AuthCodeSpotify;

use crate::connection::{FutureConnection, FutureConnectionData, PlayerConnection};
use crate::player::commands::Command;
use crate::util::client_with_token;

type Awaiting = HashMap<i32, Vec<Arc<Mutex<FutureConnectionData>>>>;

#[derive(Clone, Default)]
pub struct PlayerManager {
    players: Arc<Mutex<HashMap<i32, PlayerConnection>>>,
    awaiting: Arc<Mutex<Awaiting>>,
}

impl PlayerManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_player(&self, user_id: i32, spotify_client: AuthCodeSpotify) -> PlayerConnection {
        let player_connection = PlayerConnection::new(spotify_client);
        let mut players = self.players.lock().unwrap();
        players.insert(user_id, player_connection.clone());

        let awaiting = self.awaiting.lock().unwrap().remove(&user_id);
        if let Some(awaiting) = &awaiting {
            for a in awaiting {
                let mut lock = a.lock().unwrap();
                lock.connection = Some(player_connection.clone());
                if let Some(waker) = lock.waker.take() {
                    waker.wake();
                }
            }
        }

        player_connection
    }

    pub fn get_player_connection(&self, user_id: i32) -> Option<PlayerConnection> {
        let players = self.players.lock().unwrap();
        if let Some(player) = players.get(&user_id) {
            if player.sender.is_closed() || player.receiver.has_changed().is_err() {
                None
            } else {
                Some(player.clone())
            }
        } else {
            None
        }
    }

    pub async fn await_player_connection(&self, user_id: i32) -> PlayerConnection {
        if let Some(connection) = self.get_player_connection(user_id) {
            return connection;
        }

        let data = Arc::new(Mutex::new(FutureConnectionData {
            connection: None,
            waker: None,
        }));

        {
            let mut awaiting = self.awaiting.lock().unwrap();
            let values = awaiting.entry(user_id).or_default();
            values.push(data.clone());
        }

        FutureConnection::new(data.clone()).await
    }

    pub fn send_command(&self, user: User, command: Command) -> anyhow::Result<()> {
        if let Some(connection) = self.get_player_connection(user.id) {
            Ok(connection.sender.send(command)?)
        } else if let Command::Play { .. } = command {
            if let Some(token) = user.token {
                let spotify_client = client_with_token(token);
                let connection = self.new_player(user.id, spotify_client);
                Ok(connection.sender.send(command)?)
            } else {
                Err(anyhow!("no token"))
            }
        } else {
            Err(anyhow!("no player and command wasn't to begin playback"))
        }
    }
}
