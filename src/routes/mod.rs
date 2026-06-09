use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub mod api;

pub enum Error {
    NotFound,
    AlreadyExits,
    InvalidRequest(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
            Self::AlreadyExits => Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .body(Body::new("item already exists".to_string()))
                .unwrap(),
            Self::InvalidRequest(msg) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::new(msg))
                .unwrap(),
        }
    }
}
