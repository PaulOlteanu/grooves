use std::collections::HashMap;
use std::sync::Arc;

use grooves_player::player::commands::PlayData;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::error::GroovesResult;
use crate::player_connection::PlayerConnection;
use crate::util::spotify::client_with_token;

mod connection;

use connection::FutureConnection;

use self::connection::SharedState;

// TODO: We need some way to delete player connections when the player closes
pub struct State {
    pub db: DatabaseConnection,

    // User id to player
    pub players: Mutex<HashMap<i32, PlayerConnection>>,
    pub awaiting_player: std::sync::Mutex<HashMap<i32, Vec<Arc<std::sync::Mutex<SharedState>>>>>,
}

impl State {
    pub async fn create_player(
        &self,
        user_id: i32,
        token: rspotify::Token,
        play_data: PlayData,
    ) -> GroovesResult<PlayerConnection> {
        let client = client_with_token(token);
        let connection = PlayerConnection::new(client, play_data).await;

        {
            let mut lock = self.awaiting_player.lock()?;
            let awaiting = lock.remove(&user_id);
            if let Some(awaiting) = &awaiting {
                for a in awaiting {
                    let mut lock = a.lock()?;
                    lock.conn = Some(connection.clone());
                    if let Some(waker) = lock.waker.take() {
                        waker.wake();
                    }
                }
            }
        }

        self.players
            .lock()
            .await
            .insert(user_id, connection.clone());

        Ok(connection)
    }

    pub async fn await_player(&self, user_id: i32) -> PlayerConnection {
        FutureConnection::new(self, user_id).await.await
    }

    pub async fn get_player(&self, user_id: &i32) -> Option<PlayerConnection> {
        let player = self.players.lock().await.get(user_id).cloned();
        if let Some(player) = player {
            if player.receiver.has_changed().is_err() {
                None
            } else {
                Some(player)
            }
        } else {
            None
        }
    }
}
