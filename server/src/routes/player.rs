use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use phonos_entity::session;
use phonos_entity::user::{self, Entity as User};
use rspotify::model::SubscriptionLevel;
use rspotify::prelude::{BaseClient, Id, OAuthClient};
use rspotify::Token;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::json;

use crate::error::{PhonosError, PhonosResult};
use crate::models::spotify::SpotifyCreds;
use crate::util::generate_session_token;
use crate::util::spotify::{client_with_token, init_client};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(login).delete(logout))
        .route("/url", get(login_url))
        .route("/callback", get(callback))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<SpotifyCreds>,
) -> PhonosResult<impl IntoResponse> {
    let token = Token {
        access_token: payload.access_token,
        refresh_token: payload.refresh_token,
        // TODO: Set scopes
        ..Default::default()
    };

    let client = client_with_token(token.clone());
    client.refresh_token().await?;

    let token = client.get_token();
    let token_lock = token.lock().await;
    let token_lock = token_lock.unwrap();
    let token = token_lock.clone().unwrap();
    drop(token_lock);

    let token_value = json!(token);
    println!("{:?}", token_value);
    let token_deser: Result<Token, _> = serde_json::from_value(token_value);

    println!("{:?}", token_deser);

    let user = client.me().await?;

    if user.product.is_none() || user.product.unwrap() != SubscriptionLevel::Premium {
        return Err(PhonosError::OtherError("Must be premium user".to_owned()));
    }

    let existing_user = User::find()
        .filter(user::Column::SpotifyId.eq(user.id.id()))
        .one(&state.db)
        .await?;

    let user: user::Model = if let Some(user) = existing_user {
        let mut active_user: user::ActiveModel = user.into();
        active_user.token = Set(Some(json!(token)));
        active_user.update(&state.db).await?
    } else {
        let mut active_user = user::ActiveModel::new();
        active_user.spotify_id = Set(user.id.id().to_owned());
        active_user.token = Set(Some(json!(token)));
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

async fn login_url() -> PhonosResult<impl IntoResponse> {
    let client = init_client();
    let result = client.get_authorize_url(false)?;
    Ok(Json(json!({ "url": result })))
}