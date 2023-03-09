use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};

mod error;
mod extractors;
mod middleware;
mod models;
mod routes;
mod util;

pub struct State {
    db: DatabaseConnection,
}

type AppState = Arc<State>;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    // TODO: Make this not bad
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "phonos_axum=debug,tower_http=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let state = Arc::new(State { db });

    let router = routes::router(state.clone()).with_state(state);

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
