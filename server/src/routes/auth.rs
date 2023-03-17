use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use grooves_entity::session;
use grooves_entity::user::{self, Entity as User};
use rspotify::model::SubscriptionLevel;
use rspotify::prelude::{BaseClient, Id, OAuthClient};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use serde_json::json;

use crate::error::{GroovesError, GroovesResult};
use crate::util::generate_session_token;
use crate::util::spotify::{client_with_token, init_client};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(login).delete(logout))
        .route("/url", get(login_url))
        .route("/callback", get(callback))
}

#[derive(Deserialize)]
struct SpotifyCreds {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<SpotifyCreds>,
) -> GroovesResult<impl IntoResponse> {
    let token = rspotify::Token {
        access_token: payload.access_token,
        refresh_token: payload.refresh_token,
        // TODO: Set scopes
        ..Default::default()
    };

    let client = client_with_token(token.clone());
    client.refresh_token().await?;

    let token = {
        let token = client.get_token();
        let token = token.lock().await.unwrap();
        token.as_ref().map(|t| t.clone().into())
    };

    let user = client.me().await?;

    if user.product.is_none() || user.product.unwrap() != SubscriptionLevel::Premium {
        return Err(GroovesError::OtherError("Must be premium user".to_owned()));
    }

    let existing_user = User::find()
        .filter(user::Column::SpotifyId.eq(user.id.id()))
        .one(&state.db)
        .await?;

    let user: user::Model = if let Some(user) = existing_user {
        let mut active_user: user::ActiveModel = user.into();
        active_user.token = Set(token);
        active_user.update(&state.db).await?
    } else {
        let mut active_user = user::ActiveModel::new();
        active_user.spotify_id = Set(user.id.id().to_owned());
        active_user.token = Set(token);
        active_user.insert(&state.db).await?
    };

    let session_token = generate_session_token();

    tracing::debug!(
        "Created session: Token: {}, User ID: {}",
        session_token,
        user.id
    );

    let mut active_session = session::ActiveModel::new();
    active_session.user_id = Set(user.id);
    active_session.token = Set(session_token);
    let session = active_session.insert(&state.db).await?;

    Ok(Json(json!({ "token": session.token })))
}

async fn login_url() -> GroovesResult<impl IntoResponse> {
    let client = init_client();
    let result = client.get_authorize_url(false)?;
    Ok(Json(json!({ "url": result })))
}

async fn callback(
    Query(params): Query<HashMap<String, String>>,
) -> GroovesResult<impl IntoResponse> {
    let code = params
        .get("code")
        .ok_or(GroovesError::OtherError("Missing code".to_owned()))?;

    let client = init_client();

    client.request_token(code).await?;

    let token = client.get_token();

    if let Ok(token) = token.lock().await {
        if let Some(token) = token.as_ref() {
            return Ok(Json(
                json!({"access_token": token.access_token, "refresh_token": token.refresh_token}),
            ));
        }
    }

    Err(GroovesError::OtherError("Couldn't authenticate".to_owned()))
}

// TODO: This
async fn logout() -> impl IntoResponse {
    "TODO"
}
