use axum::Router;

use crate::app::AppState;

pub mod assets;

pub fn router() -> Router<AppState> {
    Router::new().nest("/assets", assets::router())
}
