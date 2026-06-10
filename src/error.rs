use axum::{
    Json,
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("missing authorization header")]
    MissingAuthorization,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("user does not exists")]
    UserDoesNotExists,
    #[error("asset does not exists")]
    AssetDoesNotExists,
    #[error("this username is already registered")]
    UsernameTaken,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Template(#[from] askama::Error),
    #[error(transparent)]
    Jwt(#[from] jwt_simple::Error),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> response::Response {
        let response = ErrorResponse {
            message: self.to_string(),
        };

        let status = match self {
            Self::MissingAuthorization => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::UserDoesNotExists | Self::AssetDoesNotExists => StatusCode::NOT_FOUND,
            Self::UsernameTaken => StatusCode::BAD_REQUEST,
            Self::Database(_) | Self::Template(_) | Self::Jwt(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, Json(response)).into_response()
    }
}
