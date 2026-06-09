use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};

use crate::app::AppState;

const SECRET_KEY: &str = "trust-me";

pub enum AdminError {
    NoCredentials,
    InvalidCredentials,
}

impl IntoResponse for AdminError {
    fn into_response(self) -> Response {
        match self {
            Self::NoCredentials => Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::new("no key was provided".to_string()))
                .unwrap(),
            Self::InvalidCredentials => Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::new("key mismatch".to_string()))
                .unwrap(),
        }
    }
}

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = AdminError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(key) = parts.headers.get(header::AUTHORIZATION) {
            if key.to_str().is_ok_and(|s| s == SECRET_KEY) {
                Ok(Self)
            } else {
                Err(AdminError::InvalidCredentials)
            }
        } else {
            Err(AdminError::NoCredentials)
        }
    }
}
