use axum::http::StatusCode;
use axum::response::IntoResponse;
use rspotify::model::IdError;
use rspotify::ClientError;
use tokio::sync::mpsc::error::SendError;
use tracing::debug;

#[derive(Debug)]
pub enum GroovesError {
    NotFound,
    Unauthorized,
    Forbidden,
    InvalidRequest,
    InternalError(String),
}

pub type GroovesResult<T> = Result<T, GroovesError>;

impl From<ClientError> for GroovesError {
    fn from(value: ClientError) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<IdError> for GroovesError {
    fn from(value: IdError) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<sea_orm::DbErr> for GroovesError {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<serde_json::Error> for GroovesError {
    fn from(value: serde_json::Error) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<axum::Error> for GroovesError {
    fn from(value: axum::Error) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl<T> From<SendError<T>> for GroovesError {
    fn from(value: SendError<T>) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl IntoResponse for GroovesError {
    fn into_response(self) -> axum::response::Response {
        debug!(error = ?self, "responding with error");

        match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::InvalidRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND).into_response(),
            Self::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
