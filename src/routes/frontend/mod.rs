mod login;

use axum::Router;

use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/login", login::router())
}
