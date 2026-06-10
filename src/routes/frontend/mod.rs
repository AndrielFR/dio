mod home;
mod login;

use axum::{Router, routing::get};

use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home::page))
        .nest("/login", login::router())
}
