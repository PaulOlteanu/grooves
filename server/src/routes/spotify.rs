use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use axum_macros::debug_handler;
use grooves_entity::playlist::{PlaylistElement, Song};
use grooves_entity::user;
use itertools::Itertools;
use rspotify::model::{AlbumId, SearchResult, SearchType};
use rspotify::prelude::{BaseClient, Id};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, Set};
use serde::Serialize;
use serde_json::json;
use tracing::{debug, info};

use crate::error::{GroovesError, GroovesResult};
use crate::util::spotify;
use crate::{middleware, AppState};

pub fn router(state: AppState) -> Router<AppState> {
    info!("Creating spotify api routes");

    Router::new()
        .route("/search", get(search))
        .route("/album_to_element/:album_id", get(album_to_element))
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
        .ok_or(GroovesError::InternalError("Invalid query".to_string()))?;

    let token = current_user.token.ok_or(GroovesError::Unauthorized)?.0;

    let client = spotify::client_with_token(token);

    debug!(query, "searching spotify for songs");
    let SearchResult::Tracks(songs) = client
        .search(query, SearchType::Track, None, None, None, None).await? else {
            return Err(GroovesError::InternalError(
                "Spotify search failed".to_string(),
            ));
    };

    let songs: Vec<SearchResponse> = songs
        .items
        .iter()
        .map(|song| SearchResponse {
            name: song.name.clone(),
            spotify_id: song.id.as_ref().unwrap().id().to_string(),
            image_url: get_min_image_url(&song.album.images).unwrap_or("".to_string()),
        })
        .collect();

    debug!(query, "searching spotify for albums");
    let SearchResult::Albums(albums) = client
        .search(query, SearchType::Album, None, None, None, None).await? else {
            return Err(GroovesError::InternalError(
                "Spotify search failed".to_string(),
            ));
    };

    let albums: Vec<SearchResponse> = albums
        .items
        .iter()
        .map(|album| SearchResponse {
            name: album.name.clone(),
            spotify_id: album.id.as_ref().unwrap().id().to_string(),
            image_url: get_min_image_url(&album.images).unwrap_or("".to_string()),
        })
        .collect();

    let mut active_user = user::ActiveModel::new();
    active_user.id = Set(current_user.id);
    let token = client.get_token();
    let token = token.lock().await.unwrap();
    let token = token.as_ref().map(|t| t.clone().into());
    active_user.token = Set(token);

    active_user.update(&state.db).await?;

    Ok(Json(json!({"songs": songs, "albums": albums})))
}

#[debug_handler]
async fn album_to_element(
    State(state): State<AppState>,
    Extension(current_user): Extension<user::Model>,
    Path(album_id): Path<String>,
) -> GroovesResult<impl IntoResponse> {
    let token = current_user.token.ok_or(GroovesError::Unauthorized)?.0;

    let client = spotify::client_with_token(token);

    debug!(album_id, "getting album from spotify");
    let album = client.album(AlbumId::from_id(album_id)?).await?;
    let image_url = get_max_image_url(&album.images).unwrap_or("".to_string());
    let artists = album.artists.iter().map(|a| &a.name).join(", ");

    let songs: Vec<Song> = album
        .tracks
        .items
        .iter()
        .map(|s| Song {
            name: s.name.clone(),
            image_url: image_url.clone(),
            artists: artists.clone(),
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

    let response: PlaylistElement = PlaylistElement {
        name: album.name,
        artists,
        image_url,
        songs,
    };

    Ok(Json(response))
}

fn get_min_image_url(images: &[rspotify::model::Image]) -> Option<String> {
    images
        .iter()
        .min_by_key(|a| a.height.unwrap_or(0))
        .map(|img| img.url.clone())
}

fn get_max_image_url(images: &[rspotify::model::Image]) -> Option<String> {
    images
        .iter()
        .max_by_key(|a| a.height.unwrap_or(0))
        .map(|img| img.url.clone())
}
