use std::sync::PoisonError;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use rspotify::model::IdError;
use rspotify::ClientError;
use tokio::sync::mpsc::error::SendError;

pub enum PhonosError {
    NotFound,
    Unauthorized,
    OtherError(String),
}

pub type PhonosResult<T> = Result<T, PhonosError>;

impl From<ClientError> for PhonosError {
    fn from(value: ClientError) -> Self {
        Self::OtherError(value.to_string())
    }
}

impl From<IdError> for PhonosError {
    fn from(value: IdError) -> Self {
        Self::OtherError(value.to_string())
    }
}

impl From<sea_orm::DbErr> for PhonosError {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::OtherError(value.to_string())
    }
}

impl From<serde_json::Error> for PhonosError {
    fn from(value: serde_json::Error) -> Self {
        Self::OtherError(value.to_string())
    }
}

impl From<axum::Error> for PhonosError {
    fn from(value: axum::Error) -> Self {
        Self::OtherError(value.to_string())
    }
}

impl<T> From<PoisonError<T>> for PhonosError {
    fn from(_value: PoisonError<T>) -> Self {
        Self::OtherError("Tried to lock poisoned mutex".to_string())
    }
}

impl<T> From<SendError<T>> for PhonosError {
    fn from(value: SendError<T>) -> Self {
        Self::OtherError(value.to_string())
    }
}

impl IntoResponse for PhonosError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND).into_response(),
            Self::OtherError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
