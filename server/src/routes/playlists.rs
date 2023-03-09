use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use axum_macros::debug_handler;
use phonos_entity::playlist::{self, Entity as Playlist};
use phonos_entity::user;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use serde_json::json;

use crate::error::{PhonosError, PhonosResult};
use crate::models::playlist::PlaylistElement;
use crate::{middleware, AppState};

pub fn router(state: AppState) -> Router<AppState> {
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
    Extension(current_user): Extension<user::Model>,
) -> PhonosResult<impl IntoResponse> {
    let user_playlists = Playlist::find()
        .filter(playlist::Column::OwnerId.eq(current_user.id))
        .all(&state.db)
        .await?;

    Ok(Json(user_playlists))
}

#[derive(Deserialize, Clone, Debug)]
struct CreatePlaylist {
    name: String,
    elements: Vec<PlaylistElement>,
}

#[debug_handler]
async fn create_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Json(payload): Json<CreatePlaylist>,
) -> PhonosResult<impl IntoResponse> {
    let mut playlist = playlist::ActiveModel::new();
    playlist.name = Set(payload.name);
    playlist.elements = Set(Some(json!(payload.elements)));
    playlist.owner_id = Set(current_user.id);

    let result = playlist.insert(&state.db).await?;
    Ok(Json(result))
}

async fn get_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Path(playlist_id): Path<i32>,
) -> PhonosResult<impl IntoResponse> {
    if let Some(user_playlist) = Playlist::find_by_id(playlist_id).one(&state.db).await? {
        if user_playlist.owner_id != current_user.id {
            Err(PhonosError::Unauthorized)
        } else {
            Ok(Json(user_playlist))
        }
    } else {
        Err(PhonosError::NotFound)
    }
}

async fn update_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Path(playlist_id): Path<i32>,
    Json(payload): Json<CreatePlaylist>,
) -> PhonosResult<impl IntoResponse> {
    if let Some(user_playlist) = Playlist::find_by_id(playlist_id).one(&state.db).await? {
        if user_playlist.owner_id != current_user.id {
            Err(PhonosError::Unauthorized)
        } else {
            let mut active_playlist: playlist::ActiveModel = user_playlist.into();
            active_playlist.name = Set(payload.name);
            active_playlist.elements = Set(Some(json!(payload.elements)));
            let playlist = active_playlist.update(&state.db).await?;

            Ok(Json(playlist))
        }
    } else {
        Err(PhonosError::NotFound)
    }
}

async fn delete_playlist(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Path(playlist_id): Path<i32>,
) -> PhonosResult<impl IntoResponse> {
    if let Some(user_playlist) = Playlist::find_by_id(playlist_id).one(&state.db).await? {
        if user_playlist.owner_id != current_user.id {
            Err(PhonosError::Unauthorized)
        } else {
            Playlist::delete_by_id(playlist_id).exec(&state.db).await?;
            Ok(StatusCode::OK)
        }
    } else {
        Err(PhonosError::NotFound)
    }
}