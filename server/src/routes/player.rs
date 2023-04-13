use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_macros::debug_handler;
use grooves_entity::session::{self, Entity as Session};
use grooves_entity::user::{self, Entity as User};
use grooves_player::player::commands::Command;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::error::{GroovesError, GroovesResult};
use crate::{middleware, AppState};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new().route("/", get(connect)).route(
        "/",
        post(command_handler).route_layer(axum::middleware::from_fn_with_state(
            state,
            middleware::auth::auth,
        )),
    )
}

async fn connect(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connect(socket, state))
}

async fn handle_connect(mut socket: WebSocket, state: AppState) {
    // Get the current user
    let user = if let Ok(Some(user)) = authorize(&mut socket, &state.db).await {
        user
    } else {
        return;
    };

    loop {
        let manager = &state.player_manager;

        let connection = manager.await_player_connection(user.id).await;
        let mut receiver = connection.receiver;

        // Loop and wait for player state updates, begin sending them
        while receiver.changed().await.is_ok() {
            let m = serde_json::to_string(&*receiver.borrow());
            if let Ok(msg) = m {
                if socket.send(Message::Text(msg)).await.is_err() {
                    return;
                }
            } else if socket.send(Message::Text("".to_string())).await.is_err() {
                return;
            };
        }

        if socket.send(Message::Binary(Vec::new())).await.is_err() {
            return;
        }
    }
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
        Err(GroovesError::OtherError("command failed".to_string()))
    }
}

async fn authorize(
    socket: &mut WebSocket,
    db: &DatabaseConnection,
) -> GroovesResult<Option<user::Model>> {
    if let Some(Ok(Message::Text(token))) = socket.recv().await {
        let result: Option<(_, Option<user::Model>)> = Session::find()
            .filter(session::Column::Token.eq(token))
            .find_also_related(User)
            .one(db)
            .await?;

        if let Some((_, user)) = result {
            return Ok(user);
        }
    }

    Ok(None)
}
