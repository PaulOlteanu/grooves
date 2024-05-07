use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use axum_macros::debug_handler;
use grooves_model::{PlaylistElement, User};
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
    // let user_playlists = Playlist::find()
    //     .filter(playlist::Column::OwnerId.eq(current_user.id))
    //     .all(&state.db_pool)
    //     .await?;

    // Ok(Json(user_playlists))
    todo!();
    Ok("")
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
    // let mut playlist = playlist::ActiveModel::new();
    // playlist.name = Set(payload.name);
    // playlist.elements = Set(payload.elements.into());
    // playlist.owner_id = Set(current_user.id);

    // let result = playlist.insert(&state.db_pool).await?;
    // Ok(Json(result))
    todo!();
    Ok("")
}

async fn get_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(playlist_id): Path<i32>,
) -> GroovesResult<impl IntoResponse> {
    // if let Some(user_playlist) = Playlist::find_by_id(playlist_id)
    //     .one(&state.db_pool)
    //     .await?
    // {
    //     if user_playlist.owner_id != current_user.id {
    //         Err(GroovesError::Forbidden)
    //     } else {
    //         Ok(Json(user_playlist))
    //     }
    // } else {
    //     Err(GroovesError::NotFound)
    // }
    todo!();
    Ok("")
}

async fn update_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(playlist_id): Path<i32>,
    Json(payload): Json<CreatePlaylist>,
) -> GroovesResult<impl IntoResponse> {
    // if let Some(user_playlist) = Playlist::find_by_id(playlist_id)
    //     .one(&state.db_pool)
    //     .await?
    // {
    //     if user_playlist.owner_id != current_user.id {
    //         Err(GroovesError::Forbidden)
    //     } else {
    //         let mut active_playlist: playlist::ActiveModel = user_playlist.into();
    //         active_playlist.name = Set(payload.name);
    //         active_playlist.elements = Set(payload.elements.into());
    //         let playlist = active_playlist.update(&state.db_pool).await?;

    //         Ok(Json(playlist))
    //     }
    // } else {
    //     Err(GroovesError::NotFound)
    // }
    todo!();
    Ok("")
}

async fn delete_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(playlist_id): Path<i32>,
) -> GroovesResult<impl IntoResponse> {
    // if let Some(user_playlist) = Playlist::find_by_id(playlist_id)
    //     .one(&state.db_pool)
    //     .await?
    // {
    //     if user_playlist.owner_id != current_user.id {
    //         Err(GroovesError::Forbidden)
    //     } else {
    //         Playlist::delete_by_id(playlist_id)
    //             .exec(&state.db_pool)
    //             .await?;
    //         Ok(StatusCode::OK)
    //     }
    // } else {
    //     Err(GroovesError::NotFound)
    // }
    todo!();
    Ok("")
}
