use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::debug;

#[derive(Debug)]
pub enum GroovesError {
    NotFound,
    Unauthorized,
    Forbidden,
    InvalidRequest,
    InternalError(anyhow::Error),
}

pub type GroovesResult<T> = Result<T, GroovesError>;

impl<T> From<T> for GroovesError
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn from(value: T) -> Self {
        Self::InternalError(value.into())
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
            Self::InternalError(error) => {
                debug!(error_source = error.source(), "error source");
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
        }
    }
}
