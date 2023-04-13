use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use grooves_entity::user;
use rspotify::AuthCodeSpotify;
use sea_orm::DatabaseConnection;

use crate::connection::{FutureConnection, FutureConnectionData, PlayerConnection};
use crate::player::commands::Command;
use crate::util::client_with_token;

pub struct PlayerManager {
    db: DatabaseConnection,
    players: Mutex<HashMap<i32, PlayerConnection>>,
    awaiting: Mutex<HashMap<i32, Vec<Arc<Mutex<FutureConnectionData>>>>>,
}

impl PlayerManager {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            players: Mutex::new(HashMap::new()),
            awaiting: Mutex::new(HashMap::new()),
        }
    }

    pub fn new_player(&self, user_id: i32, spotify_client: AuthCodeSpotify) -> PlayerConnection {
        let player_connection = PlayerConnection::new(spotify_client, self.db.clone());
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
            let values = awaiting.entry(user_id).or_insert_with(Vec::new);
            values.push(data.clone());
        }

        FutureConnection::new(data.clone()).await
    }

    pub fn send_command(&self, user: user::Model, command: Command) -> Result<(), ()> {
        if let Some(connection) = self.get_player_connection(user.id) {
            connection.sender.send(command).or(Err(()))
        } else if let Command::Play { .. } = command {
            if let Some(token) = user.token {
                let spotify_client = client_with_token(token.0);
                let connection = self.new_player(user.id, spotify_client);
                connection.sender.send(command).or(Err(()))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
