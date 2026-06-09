use std::{collections::HashMap, sync::Arc};

use axum::Router;
use tokio::{net::TcpListener, sync::RwLock};

use crate::{models::Asset, routes};

#[derive(Clone)]
pub struct AppState {
    pub assets: Arc<RwLock<HashMap<i64, Asset>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}

pub struct App;

impl App {
    pub async fn serve() -> color_eyre::Result<()> {
        let addr = "localhost:8080";
        let listener = TcpListener::bind(&addr).await?;

        let router = Router::new()
            .nest("/api", routes::api::router())
            .with_state(AppState::new());

        tracing::info!("listening at {addr:?}");
        axum::serve(listener, router).await?;

        Ok(())
    }
}
