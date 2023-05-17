use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use grooves_entity::session;
use grooves_entity::user::{self, Entity as User};
use rspotify::model::SubscriptionLevel;
use rspotify::prelude::{BaseClient, Id, OAuthClient};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
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

    if !matches!(user.product, Some(SubscriptionLevel::Premium)) {
        return Err(GroovesError::InternalError(
            "Must be premium user".to_owned(),
        ));
    }

    let existing_user = User::find()
        .filter(user::Column::SpotifyId.eq(user.id.id()))
        .one(&state.db_pool)
        .await?;

    let token = {
        let token = client.get_token();
        let token = token.lock().await.unwrap();
        token.as_ref().map(|t| t.clone().into())
    };

    let user: user::Model = if let Some(user) = existing_user {
        let mut active_user: user::ActiveModel = user.into();
        active_user.token = Set(token);
        active_user.update(&state.db_pool).await?
    } else {
        let mut active_user = user::ActiveModel::new();
        active_user.spotify_id = Set(user.id.id().to_owned());
        active_user.token = Set(token);
        active_user.insert(&state.db_pool).await?
    };

    let session_token = generate_session_token();

    tracing::debug!(session_token, user_id = user.id, "Created session");

    let mut active_session = session::ActiveModel::new();
    active_session.user_id = Set(user.id);
    active_session.token = Set(session_token);
    let session = active_session.insert(&state.db_pool).await?;

    Ok(Json(json!({ "token": session.token })))
}

// TODO: This
async fn logout() -> impl IntoResponse {
    "TODO"
}
