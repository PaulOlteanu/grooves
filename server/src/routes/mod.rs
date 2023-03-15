use axum::http::Method;
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

mod auth;
mod player;
mod playlists;
mod spotify;

pub fn router(state: AppState) -> Router<AppState> {
    let origins = [
        "http://localhost:5173".parse().unwrap(),
        "https://localhost:5173".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(origins);

    Router::<AppState>::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth::router())
        .nest("/player", player::router())
        .nest("/playlists", playlists::router(state.clone()))
        .nest("/spotify", spotify::router(state))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
