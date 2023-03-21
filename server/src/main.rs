use std::collections::HashMap;
use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};
use tokio::sync::Mutex;

mod error;
mod extractors;
mod middleware;
mod player_connection;
mod routes;
mod util;

use player_connection::PlayerConnection;
use tracing_subscriber::{prelude::*, util::SubscriberInitExt};
use util::spotify::client_with_token;

// TODO: We need some way to delete player connections when the player closes
pub struct State {
    db: DatabaseConnection,
    // User id to player
    players: Mutex<HashMap<i32, Arc<Mutex<PlayerConnection>>>>,
}

impl State {
    pub async fn get_or_create_player(
        &self,
        user_id: i32,
        user_token: rspotify::Token,
    ) -> Arc<Mutex<PlayerConnection>> {
        let client = client_with_token(user_token);
        let mut players = self.players.lock().await;
        match players.get(&user_id) {
            Some(connection) => connection.clone(),
            None => players
                .entry(user_id)
                .or_insert(Arc::new(Mutex::new(PlayerConnection::new(client))))
                .clone(),
        }
    }
}

type AppState = Arc<State>;

#[tokio::main]
async fn main() {
    // std::env::set_var("RUST_LOG", "debug");
    // tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    // TODO: Make this not bad
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "grooves_axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: DatabaseConnection = Database::connect(database_url)
        .await
        .expect("couldn't connect to database");

    let state = Arc::new(State {
        db,
        players: Mutex::new(HashMap::new()),
    });

    let router = routes::router(state.clone()).with_state(state);

    axum::Server::bind(&"0.0.0.0:4000".parse().expect("couldn't create binding"))
        .serve(router.into_make_service())
        .await
        .expect("couldn't create server");
}
