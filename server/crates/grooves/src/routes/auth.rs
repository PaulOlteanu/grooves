use anyhow::anyhow;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use grooves_model::{Session, User};
use rspotify::model::SubscriptionLevel;
use rspotify::prelude::{BaseClient, Id, OAuthClient};
use serde::Deserialize;
use serde_json::json;
use tracing::{info, trace};

use crate::error::{GroovesError, GroovesResult};
use crate::util::generate_session_token;
use crate::util::spotify::init_client;
use crate::AppState;

pub fn router() -> Router<AppState> {
    info!("Creating auth routes");
    Router::new().route("/", post(login).delete(logout))
}

#[derive(Deserialize)]
struct AuthRequest {
    pub code: String,
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> GroovesResult<impl IntoResponse> {
    let mut client = init_client();
    client.oauth.redirect_uri = format!("{}/callback", std::env::var("FRONTEND_URL").unwrap());

    trace!("requesting token");
    client.request_token(&payload.code).await?;

    trace!("getting current user");
    let user = client.current_user().await?;

    // TODO: This should be a different error
    if !matches!(user.product, Some(SubscriptionLevel::Premium)) {
        return Err(GroovesError::InternalError(anyhow!("Must be premium user")));
    }

    let existing_user: Option<User> = sqlx::query_as(r#"SELECT * FROM "user" WHERE spotify_id=$1"#)
        .bind(user.id.id())
        .fetch_optional(&state.db_pool)
        .await?;

    let token = {
        let token = client.get_token();
        let token = token.lock().await.unwrap();
        token.as_ref().map(|t| t.clone())
    };

    let user: User = if let Some(user) = existing_user {
        sqlx::query_as(r#"UPDATE "user" SET token=$1 WHERE id=2 RETURNING *"#)
            .bind(sqlx::types::Json::from(token))
            .bind(user.id)
            .fetch_one(&state.db_pool)
            .await?
    } else {
        sqlx::query_as(r#"INSERT INTO "user" (spotify_id, token) VALUES ($1, $2) RETURNING *"#)
            .bind(user.id.id())
            .bind(sqlx::types::Json::from(token))
            .fetch_one(&state.db_pool)
            .await?
    };

    let session_token = generate_session_token();

    tracing::info!(session_token, user_id = user.id, "Created session");

    let session: Session =
        sqlx::query_as(r#"INSERT INTO session (user_id, token) VALUES ($1, $2) RETURNING *"#)
            .bind(user.id)
            .bind(session_token)
            .fetch_one(&state.db_pool)
            .await?;

    Ok(Json(json!({ "token": session.token })))
}

// TODO: This
async fn logout() -> impl IntoResponse {
    "TODO"
}
