use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use axum_macros::debug_handler;
use grooves_model::{Playlist, PlaylistElement, User};
use serde::Deserialize;
use tracing::info;

use crate::error::{GroovesError, GroovesResult};
use crate::{middleware, AppState};

pub fn router(state: AppState) -> Router<AppState> {
    info!("Creating playlist routes");

    Router::new()
        .route("/", get(get_playlists).post(create_playlist))
        .route(
            "/:playlistId",
            get(get_playlist)
                .put(update_playlist)
                .delete(delete_playlist),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            middleware::auth::auth,
        ))
}

async fn get_playlists(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
) -> GroovesResult<impl IntoResponse> {
    let playlists: Vec<Playlist> = sqlx::query_as("SELECT * FROM playlist WHERE owner_id = $1")
        .bind(current_user.id)
        .fetch_all(&state.db_pool)
        .await?;

    Ok(Json(playlists))
}

#[derive(Deserialize, Clone, Debug)]
struct CreatePlaylist {
    name: String,
    elements: Vec<PlaylistElement>,
}

#[debug_handler]
async fn create_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<CreatePlaylist>,
) -> GroovesResult<impl IntoResponse> {
    let playlist: Playlist = sqlx::query_as(
        "INSERT INTO playlist (name, owner_id, elements) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(payload.name)
    .bind(current_user.id)
    .bind(sqlx::types::Json::from(payload.elements))
    .fetch_one(&state.db_pool)
    .await?;

    Ok(Json(playlist))
}

async fn get_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(playlist_id): Path<i32>,
) -> GroovesResult<impl IntoResponse> {
    let playlist: Playlist =
        sqlx::query_as("SELECT * FROM playlist WHERE id = $1 AND owner_id = $2")
            .bind(playlist_id)
            .bind(current_user.id)
            .fetch_optional(&state.db_pool)
            .await?
            .ok_or(GroovesError::NotFound)?;

    Ok(Json(playlist))
}

async fn update_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(playlist_id): Path<i32>,
    Json(payload): Json<CreatePlaylist>,
) -> GroovesResult<impl IntoResponse> {
    let playlist: Playlist = sqlx::query_as(
        r#"UPDATE playlist
            SET name = $1, elements = $2
            WHERE id = $3 AND owner_id = $4
            RETURNING *"#,
    )
    .bind(payload.name)
    .bind(sqlx::types::Json::from(payload.elements))
    .bind(playlist_id)
    .bind(current_user.id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or(GroovesError::NotFound)?;

    Ok(Json(playlist))
}

async fn delete_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(playlist_id): Path<i32>,
) -> GroovesResult<impl IntoResponse> {
    let res = sqlx::query("DELETE FROM playlist WHERE id = $1 AND owner_id = $2")
        .bind(playlist_id)
        .bind(current_user.id)
        .execute(&state.db_pool)
        .await?;

    if res.rows_affected() == 0 {
        Err(GroovesError::NotFound)
    } else {
        Ok(StatusCode::OK)
    }
}
