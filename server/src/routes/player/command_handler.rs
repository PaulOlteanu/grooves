use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use grooves_entity::playlist::Entity as Playlist;
use grooves_entity::user::{self, Token};
use grooves_player::player::commands::{Command, PlayData};
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
                    Ok(Some(Command::Play(PlayData {
                        playlist,
                        element_index,
                        song_index,
                    })))
                } else {
                    Ok(None)
                }
            }
            Self::Pause => Ok(Some(Command::Pause)),
            Self::Resume => Ok(Some(Command::Resume)),
            Self::NextSong => Ok(Some(Command::NextSong)),
            Self::PrevSong => Ok(Some(Command::PrevSong)),
            Self::NextElement => Ok(Some(Command::NextElement)),
            Self::PrevElement => Ok(Some(Command::PrevElement)),
            Self::AddToQueue => Ok(Some(Command::AddToQueue)),
            Self::RemoveFromQueue => Ok(Some(Command::RemoveFromQueue)),
            Self::Exit => Ok(Some(Command::Exit)),
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

    if let Some(command) = payload.to_player_command(state.clone()).await? {
        if let Command::Play(ref play_data) = command {
            if let Some(connection) = state.get_player(&current_user.id).await {
                connection.sender.send(command)?;
            } else {
                state
                    .create_player(current_user.id, token, play_data.clone())
                    .await?;
            }

            Ok("sent command")
        } else if let Some(connection) = state.get_player(&current_user.id).await {
            connection.sender.send(command)?;
            Ok("sent command")
        } else {
            Err(GroovesError::OtherError("no player for user".to_string()))
        }
    } else {
        Err(GroovesError::OtherError(
            "invalid player command".to_string(),
        ))
    }
}
