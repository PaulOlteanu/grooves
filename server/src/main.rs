use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use grooves_migration::{Migrator, MigratorTrait};
use grooves_player::manager::PlayerManager;
use sea_orm::{Database, DatabaseConnection};
use state::State;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod error;
mod middleware;
mod routes;
mod state;
mod util;

type AppState = Arc<State>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "grooves_axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let _ = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    let _ = std::env::var("RSPOTIFY_CLIENT_ID").expect("RSPOTIFY_CLIENT_ID must be set");
    let _ = std::env::var("RSPOTIFY_CLIENT_SECRET").expect("RSPOTIFY_CLIENT_SECRET must be set");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("couldn't connect to database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to migrate database");

    let player_db_pool: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("couldn't connect to database");

    let state = Arc::new(State {
        db,
        player_manager: PlayerManager::new(player_db_pool),
        sse_tokens: Mutex::new(HashMap::new()),
    });

    let router = routes::router(state.clone()).with_state(state);

    let port = std::env::var("GROOVES_PORT")
        .expect("GROOVES_PORT must be set")
        .parse()
        .expect("Invalid GROOVES_PORT");

    let addr = SocketAddr::new("0.0.0.0".parse().unwrap(), port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("couldn't create server");
}
