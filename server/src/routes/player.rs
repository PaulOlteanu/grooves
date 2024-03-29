use std::collections::HashMap;
use std::convert::Infallible;

use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Sse};
use axum::routing::get;
use axum::{Extension, Json, Router};
use axum_macros::debug_handler;
use futures_util::stream::Stream;
use grooves_entity::user;
use grooves_player::player::commands::Command;
use tracing::info;

use crate::error::{GroovesError, GroovesResult};
use crate::{middleware, util, AppState};

#[rustfmt::skip]
pub fn router(state: AppState) -> Router<AppState> {
    info!("Creating player routes");

    Router::new()
        .route("/", get(sse_handler).post(command_handler))
        .route("/sse_token", get(sse_token))
            .route_layer(axum::middleware::from_fn_with_state(
                state,
                middleware::auth::auth,
            ))
}

async fn sse_token(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
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
        .ok_or(GroovesError::InternalError("Missing token".to_string()))?;

    let user = state
        .sse_tokens
        .lock()
        .unwrap()
        .remove(sse_token)
        .ok_or(GroovesError::InternalError("invalid token".to_string()))?;

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
    Extension(current_user): Extension<user::Model>,
    Json(command): Json<Command>,
) -> GroovesResult<impl IntoResponse> {
    let manager = &state.player_manager;
    if manager.send_command(current_user, command).is_ok() {
        Ok("sent command")
    } else {
        Err(GroovesError::InternalError("command failed".to_string()))
    }
}
