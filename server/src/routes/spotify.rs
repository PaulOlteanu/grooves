use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use axum_macros::debug_handler;
use grooves_entity::playlist::Song;
use grooves_entity::user;
use rspotify::model::{AlbumId, SearchResult, SearchType, SimplifiedAlbum};
use rspotify::prelude::{BaseClient, Id};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, Set};
use serde::Serialize;
use serde_json::json;

use crate::error::{GroovesError, GroovesResult};
use crate::util::spotify;
use crate::{middleware, AppState};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/search", get(search))
        .route("/album_songs/:album_id", get(album_songs))
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            middleware::auth::auth,
        ))
}

#[derive(Debug, Serialize)]
struct SearchResponse {
    name: String,
    spotify_id: String,
    image_url: String,
}

#[debug_handler]
async fn search(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Query(params): Query<HashMap<String, String>>,
) -> GroovesResult<impl IntoResponse> {
    let query = params
        .get("q")
        .ok_or(GroovesError::OtherError("Invalid query".to_string()))?;

    let token = current_user.token.ok_or(GroovesError::Unauthorized)?.0;

    let client = spotify::client_with_token(token);

    let songs = client
        .search(query, SearchType::Track, None, None, None, None)
        .await?;

    let songs: Vec<SearchResponse> = if let SearchResult::Tracks(songs) = songs {
        songs
            .items
            .iter()
            .map(|song| SearchResponse {
                name: song.name.clone(),
                spotify_id: song.id.as_ref().unwrap().id().to_string(),
                image_url: get_image_url(&song.album).unwrap_or("".to_string()),
            })
            .collect()
    } else {
        return Err(GroovesError::OtherError(
            "Spotify search failed".to_string(),
        ));
    };

    let albums = client
        .search(query, SearchType::Album, None, None, None, None)
        .await?;

    let albums: Vec<SearchResponse> = if let SearchResult::Albums(albums) = albums {
        albums
            .items
            .iter()
            .map(|album| SearchResponse {
                name: album.name.clone(),
                spotify_id: album.id.as_ref().unwrap().id().to_string(),
                image_url: get_image_url(album).unwrap_or("".to_string()),
            })
            .collect()
    } else {
        return Err(GroovesError::OtherError(
            "Spotify search failed".to_string(),
        ));
    };

    let mut active_user = user::ActiveModel::new();
    active_user.id = Set(current_user.id);
    let token = client.get_token();
    let token = token.lock().await.unwrap();
    let token = token.as_ref().map(|t| t.clone().into());
    active_user.token = Set(token);

    active_user.update(&state.db).await?;

    // TODO: Store the new access token
    Ok(Json(json!({"songs": songs, "albums": albums})))
}

#[debug_handler]
async fn album_songs(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Path(album_id): Path<String>,
) -> GroovesResult<impl IntoResponse> {
    let token = current_user.token.ok_or(GroovesError::Unauthorized)?.0;

    let client = spotify::client_with_token(token);

    let album = client.album(AlbumId::from_id(album_id)?).await?;

    let songs: Vec<Song> = album
        .tracks
        .items
        .iter()
        .map(|s| Song {
            name: s.name.clone(),
            spotify_id: s.id.as_ref().unwrap().clone(),
        })
        .collect();

    let mut active_user = user::ActiveModel::new();
    active_user.id = Set(current_user.id);
    let token = client.get_token();
    let token = token.lock().await.unwrap();
    let token = token.as_ref().map(|t| t.clone().into());
    active_user.token = Set(token);

    active_user.update(&state.db).await?;

    Ok(Json(songs))
}

fn get_image_url(album: &SimplifiedAlbum) -> Option<String> {
    album
        .images
        .iter()
        .min_by_key(|a| a.height.unwrap_or(0))
        .map(|img| img.url.clone())
}
