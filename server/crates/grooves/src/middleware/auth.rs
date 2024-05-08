use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use grooves_model::User;
use tracing::warn;

use crate::error::{GroovesError, GroovesResult};
use crate::AppState;

pub async fn auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> GroovesResult<Response> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
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

    let user: User = sqlx::query_as(r#"SELECT "user".* FROM session JOIN "user" ON session.user_id = "user".id WHERE session.token=$1"#)
        .bind(auth_token)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or(GroovesError::Unauthorized)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
