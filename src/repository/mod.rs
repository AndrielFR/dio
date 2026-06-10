pub mod assets;
pub mod users;

use std::convert::Infallible;

use axum::{extract::FromRequestParts, http::request::Parts};
use sqlx::PgPool;

use crate::app::AppState;

pub struct Repository {
    pool: PgPool,
}

impl FromRequestParts<AppState> for Repository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            pool: state.pool.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for Repository {
    fn from(pool: PgPool) -> Self {
        Self { pool }
    }
}
