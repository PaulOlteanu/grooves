use std::collections::HashMap;
use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};
use state::State;
use tokio::sync::Mutex;

mod error;
mod extractors;
mod middleware;
mod player_connection;
mod routes;
mod state;
mod util;

use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;

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
        awaiting_player: std::sync::Mutex::new(HashMap::new()),
    });

    let router = routes::router(state.clone()).with_state(state);

    axum::Server::bind(&"0.0.0.0:4000".parse().expect("couldn't create binding"))
        .serve(router.into_make_service())
        .await
        .expect("couldn't create server");
}
