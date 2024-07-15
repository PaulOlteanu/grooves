use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use grooves_player::manager::PlayerManager;
use sqlx::pool::PoolOptions;
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, PgPool};
use state::State;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod error;
mod middleware;
mod routes;
mod state;
mod util;

type AppState = Arc<State>;

#[tokio::main]
async fn main() {
    let filter = match std::env::var("RUST_LOG").as_deref() {
        Ok("TRACE") => "trace",
        Ok("DEBUG") => {
            "grooves_server=debug,grooves_player=debug,tower_http=debug,axum::rejection=trace"
        }
        Ok("INFO") => "info,tower_http=warn,axum::rejection=info,rspotify_http=warn,rspotify=warn",
        Ok("WARN") => "warn",
        Ok("ERROR") => "error",
        _ => "grooves_server=debug,grooves_player=debug,tower_http=debug,axum::rejection=trace",
    };

    let filter = EnvFilter::new(filter);

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let _ = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    let _ = std::env::var("RSPOTIFY_CLIENT_ID").expect("RSPOTIFY_CLIENT_ID must be set");
    let _ = std::env::var("RSPOTIFY_CLIENT_SECRET").expect("RSPOTIFY_CLIENT_SECRET must be set");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection_options = PgConnectOptions::from_str(&database_url)
        .expect("valid database url")
        .log_statements(tracing::log::LevelFilter::Debug);

    info!("webserver connecting to database");

    let pool: PgPool = PoolOptions::new()
        .max_connections(16)
        .connect_with(connection_options)
        .await
        .expect("connection to postgres");

    let state = Arc::new(State {
        db_pool: pool,
        player_manager: PlayerManager::new(),
        sse_tokens: Mutex::new(HashMap::new()),
    });

    let router = routes::router(state.clone()).with_state(state);

    let port = std::env::var("GROOVES_PORT")
        .expect("GROOVES_PORT must be set")
        .parse()
        .expect("Invalid GROOVES_PORT");

    let addr = SocketAddr::new("0.0.0.0".parse().unwrap(), port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router)
        .await
        .expect("couldn't create server");
}
