use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};

use crate::{app::AppState, error::AppError};

const SECRET_KEY: &str = "trust-me";

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(key) = parts.headers.get(header::AUTHORIZATION) {
            if key.to_str().is_ok_and(|s| s == SECRET_KEY) {
                Ok(Self)
            } else {
                Err(AppError::InvalidCredentials)
            }
        } else {
            Err(AppError::MissingAuthorization)
        }
    }
}
