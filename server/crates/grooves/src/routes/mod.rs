use axum::http::{HeaderValue, Method};
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

mod auth;
pub mod player;
mod playlists;
mod spotify;

pub fn router(state: AppState) -> Router<AppState> {
    let frontend_url = std::env::var("FRONTEND_URL").unwrap();

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(
            frontend_url
                .parse::<HeaderValue>()
                .expect("Failed to get frontend url"),
        );

    Router::<AppState>::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth::router())
        .nest("/player", player::router(state.clone()))
        .nest("/playlists", playlists::router(state.clone()))
        .nest("/spotify", spotify::router(state))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
