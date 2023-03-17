use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use grooves_entity::playlist::Entity as Playlist;
use grooves_entity::user::{self, Token};
use grooves_player::player::commands::Command;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::error::{GroovesError, GroovesResult};
use crate::AppState;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CommandRequest {
    Play {
        playlist_id: i32,
        element_index: Option<usize>,
        song_index: Option<usize>,
    },
    Pause,
    Resume,
    NextSong,
    PrevSong,
    NextElement,
    PrevElement,
    AddToQueue,
    RemoveFromQueue,
    Exit,
}

impl CommandRequest {
    pub async fn to_player_command(self, state: AppState) -> GroovesResult<Option<Command>> {
        match self {
            Self::Play {
                playlist_id,
                element_index,
                song_index,
            } => {
                if let Some(playlist) = Playlist::find_by_id(playlist_id).one(&state.db).await? {
                    Ok(Some(Command::Play {
                        playlist,
                        element_index,
                        song_index,
                    }))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

#[debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Json(payload): Json<CommandRequest>,
) -> GroovesResult<impl IntoResponse> {
    let token = if let Some(Token(token)) = current_user.token {
        token
    } else {
        return Err(GroovesError::OtherError(
            "user missing token (please reauthenticate)".to_string(),
        ));
    };

    let connection = state.get_or_create_player(current_user.id, token).await;

    if let Some(command) = payload.to_player_command(state.clone()).await? {
        let sender = {
            let lock = connection.lock().await;
            lock.sender.clone()
        };
        sender.send(command)?;

        Ok("sent command")
    } else {
        Err(GroovesError::OtherError(
            "couldn't create command".to_string(),
        ))
    }
}
