use std::env;

use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        let database_url = env::var("DATABASE_URL")?;
        let pool = PgPool::connect(&database_url).await?;

        Ok(Self { pool })
    }
}

pub struct App;

impl App {
    pub async fn serve() -> color_eyre::Result<()> {
        let addr = "localhost:8080";
        let listener = TcpListener::bind(&addr).await?;

        let state = AppState::new().await?;
        let router = Router::new()
            .nest("/api", routes::api::router())
            .merge(routes::frontend::router())
            .with_state(state);

        tracing::info!("listening at {addr:?}");
        axum::serve(listener, router).await?;

        Ok(())
    }
}
