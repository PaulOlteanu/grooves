use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use phonos_entity::playlist::{self, Entity as Playlist};
use phonos_entity::session::{self, Entity as Session};
use phonos_entity::user::{self, Entity as User};
use rspotify::model::SubscriptionLevel;
use rspotify::prelude::{BaseClient, Id, OAuthClient};
use rspotify::Token;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};

use crate::error::{PhonosError, PhonosResult};
use crate::util::spotify::{client_with_token, init_client};
use crate::AppState;

mod command_handler;
mod messages;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(connect).post(command_handler::handler))
}

async fn connect(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    let db = state.db.clone();
    ws.on_upgrade(move |socket| handle_connect(socket, db))
}

async fn handle_connect(mut socket: WebSocket, db: DatabaseConnection) {
    // Get the current user
    let user = if let Ok(Some(user)) = authorize(&mut socket, &db).await {
        user
    } else {
        return;
    };

    // Send the current player state because it's a new connection

    // Loop and wait for player state updates, begin sending them

    // TODO: Send player state in a loop
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                println!("Received: {:?}", msg);
                socket.send(msg).await;
            } else {
                println!("client abruptly disconnected");
                return;
            }
        }
    }
}

async fn authorize(
    socket: &mut WebSocket,
    db: &DatabaseConnection,
) -> PhonosResult<Option<user::Model>> {
    if let Some(msg) = socket.recv().await {
        if let Ok(Message::Text(token)) = msg {
            let result: Option<(_, Option<user::Model>)> = Session::find()
                .filter(session::Column::Token.eq(token))
                .find_also_related(User)
                .one(db)
                .await?;

            if let Some((_, user)) = result {
                return Ok(user);
            }
        }
    }

    Ok(None)
}
