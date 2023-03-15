use axum::extract::State;
use axum::http::{self, Request};
use axum::middleware::Next;
use axum::response::Response;
use phonos_entity::session::{self, Entity as Session};
use phonos_entity::user::{self, Entity as User};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::error::{PhonosError, PhonosResult};
use crate::AppState;

pub async fn auth<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> PhonosResult<Response> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(PhonosError::Unauthorized);
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(PhonosError::Unauthorized);
    }

    let auth_token = &auth_header[7..];

    let result = Session::find()
        .filter(session::Column::Token.eq(auth_token))
        .find_also_related(User)
        .one(&state.db)
        .await?;

    if let Some((_, Some(existing_user))) = result {
        req.extensions_mut().insert(existing_user);
        return Ok(next.run(req).await);
    }

    Err(PhonosError::Unauthorized)
}
