use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
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

    // TODO: Reimplement
    // let result = Session::find()
    //     .filter(session::Column::Token.eq(auth_token))
    //     .find_also_related(User)
    //     .one(&state.db_pool)
    //     .await?;

    let result = todo!();

    // if let Some((_, Some(existing_user))) = result {
    //     req.extensions_mut().insert(existing_user);
    //     return Ok(next.run(req).await);
    // }

    Err(GroovesError::Unauthorized)
}
