use axum::extract::State;
use axum::http::{self, Request};
use axum::middleware::Next;
use axum::response::Response;
use grooves_entity::session::{self, Entity as Session};
use grooves_entity::user::Entity as User;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::warn;

use crate::error::{GroovesError, GroovesResult};
use crate::AppState;

pub async fn auth<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> GroovesResult<Response> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let Some(auth_header) = auth_header else {
        warn!("missing authorization header");
        return Err(GroovesError::Unauthorized);
    };

    if !auth_header.starts_with("Bearer ") {
        warn!("invalid authorization header format");
        return Err(GroovesError::Unauthorized);
    }

    let auth_token = &auth_header[7..];

    let result = Session::find()
        .filter(session::Column::Token.eq(auth_token))
        .find_also_related(User)
        .one(&state.db_pool)
        .await?;

    if let Some((_, Some(existing_user))) = result {
        req.extensions_mut().insert(existing_user);
        return Ok(next.run(req).await);
    }

    Err(GroovesError::Unauthorized)
}
