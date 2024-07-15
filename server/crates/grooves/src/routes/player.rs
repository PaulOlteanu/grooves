use std::collections::HashMap;
use std::convert::Infallible;

use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Sse};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_macros::debug_handler;
use grooves_model::{Playlist, User};
use grooves_player::player::commands::Command as PlayerCommand;
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;
use tracing::info;

use crate::error::{GroovesError, GroovesResult};
use crate::{middleware, util, AppState};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
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

pub fn router(state: AppState) -> Router<AppState> {
    info!("Creating player routes");

    Router::new()
        .route("/", get(sse_handler))
        .route(
            "/",
            post(command_handler).route_layer(axum::middleware::from_fn_with_state(
                state.clone(),
                middleware::auth::auth,
            )),
        )
        .route(
            "/sse_token",
            get(sse_token).route_layer(axum::middleware::from_fn_with_state(
                state,
                middleware::auth::auth,
            )),
        )
}

async fn sse_token(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
) -> GroovesResult<impl IntoResponse> {
    let token = util::generate_session_token();
    state
        .sse_tokens
        .lock()
        .unwrap()
        .insert(token.clone(), current_user);
    Ok(token)
}

async fn sse_handler(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> GroovesResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let sse_token = params
        .get("token")
        .ok_or(GroovesError::InternalError(anyhow!("missing token")))?;

    let user = state
        .sse_tokens
        .lock()
        .unwrap()
        .remove(sse_token)
        .ok_or(GroovesError::InternalError(anyhow!("invalid token")))?;

    let stream = async_stream::stream! {
        loop {
            let manager = &state.player_manager;
            let connection = manager.await_player_connection(user.id).await;
            let mut receiver = connection.receiver;

            while receiver.changed().await.is_ok() {
                let m = serde_json::to_string(&*receiver.borrow_and_update());

                if let Ok(msg) = m {
                    yield Ok(Event::default().data(msg));
                } else {
                    yield Ok(Event::default().data(""));
                };
            }

            yield Ok(Event::default().data(""));
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

#[debug_handler]
pub async fn command_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Json(command): Json<Command>,
) -> GroovesResult<impl IntoResponse> {
    let manager = &state.player_manager;

    let player_command = match command {
        Command::Play {
            playlist_id,
            element_index,
            song_index,
        } => {
            let playlist: Playlist =
                sqlx::query_as("SELECT * FROM playlist WHERE id = $1 AND owner_id = $2")
                    .bind(playlist_id)
                    .bind(current_user.id)
                    .fetch_optional(&state.db_pool)
                    .await?
                    .ok_or(GroovesError::NotFound)?;

            PlayerCommand::Play {
                playlist,
                element_index,
                song_index,
            }
        }
        Command::Pause => PlayerCommand::Pause,
        Command::Resume => PlayerCommand::Resume,
        Command::NextSong => PlayerCommand::NextSong,
        Command::PrevSong => PlayerCommand::PrevSong,
        Command::NextElement => PlayerCommand::NextElement,
        Command::PrevElement => PlayerCommand::PrevElement,
        Command::AddToQueue => PlayerCommand::AddToQueue,
        Command::RemoveFromQueue => PlayerCommand::RemoveFromQueue,
        Command::Exit => PlayerCommand::Exit,
    };

    if manager.send_command(current_user, player_command).is_ok() {
        Ok("sent command")
    } else {
        Err(GroovesError::InternalError(anyhow!("command failed")))
    }
}
