use axum::{
    Json,
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing Authorization Header")]
    MissingAuthorization,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Asset does not exists")]
    AssetDoesNotExists,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
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
            Self::AssetDoesNotExists => StatusCode::NOT_FOUND,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(response)).into_response()
    }
}
