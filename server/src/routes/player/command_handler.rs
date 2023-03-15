use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use phonos_entity::user;
use phonos_player::player;
use tokio::sync::Mutex;

use crate::error::{PhonosError, PhonosResult};
use crate::player_connection::PlayerConnection;
use crate::util::spotify::client_with_token;
use crate::AppState;

#[debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Json(payload): Json<player::commands::Command>,
) -> PhonosResult<impl IntoResponse> {
    let connection = {
        let players = state.players.read().await;

        if let Some(connection) = players.get(&current_user.id) {
            Some(connection.clone())
        } else {
            None
        }
    };

    let player_connection = if let Some(player_connection) = connection {
        player_connection
    } else {
        if let Some(token) = current_user.token {
            if let Ok(token) = serde_json::from_value(token) {
                let spotify_client = client_with_token(token);
                let connection = Arc::new(Mutex::new(PlayerConnection::new(spotify_client)));
                let mut players = state.players.write().await;
                players.insert(current_user.id, connection.clone());

                connection
            } else {
                return Err(PhonosError::OtherError(
                    "user missing token (please reauthenticate)".to_string(),
                ));
            }
        } else {
            return Err(PhonosError::OtherError(
                "user missing token (please reauthenticate)".to_string(),
            ));
        }
    };

    player_connection.lock().await.sender.send(payload).await?;

    Ok("sent command")
}
