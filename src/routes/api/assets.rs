use axum::{Json, Router, extract::State, routing::get};

use crate::{
    app::AppState,
    auth::Admin,
    dto::{CreateAsset, DeleteAsset, UpdateAsset},
    models::Asset,
    routes,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list).post(create).delete(delete).patch(update))
}

#[tracing::instrument(skip_all)]
pub async fn list(state: State<AppState>) -> Json<Vec<Asset>> {
    let assets = state.assets.read().await;

    Json(assets.values().cloned().collect())
}

#[tracing::instrument(skip_all)]
pub async fn create(
    _admin: Admin,
    state: State<AppState>,
    Json(request): Json<CreateAsset>,
) -> Result<Json<Asset>, routes::Error> {
    let mut assets = state.assets.write().await;

    if !assets.values().into_iter().any(|a| a.name == request.name) {
        let new_id = assets.values().map(|a| a.id).max().unwrap_or(0) + 1;
        let new_asset = Asset {
            id: new_id,
            name: request.name,
            unit_value: request.unit_value,
        };
        assets.insert(new_id, new_asset.clone());

        Ok(Json(new_asset))
    } else {
        Err(routes::Error::AlreadyExits)
    }
}

#[tracing::instrument(skip_all)]
pub async fn delete(
    _admin: Admin,
    state: State<AppState>,
    Json(request): Json<DeleteAsset>,
) -> Result<Json<Asset>, routes::Error> {
    let mut assets = state.assets.write().await;

    let mut extracted = assets.extract_if(|id, _| *id == request.id);
    if let Some((_, asset)) = extracted.next() {
        Ok(Json(asset))
    } else {
        Err(routes::Error::NotFound)
    }
}

#[tracing::instrument(skip_all)]
pub async fn update(
    _admin: Admin,
    state: State<AppState>,
    Json(request): Json<UpdateAsset>,
) -> Result<Json<Asset>, routes::Error> {
    let mut assets = state.assets.write().await;

    if let Some(asset) = assets.get_mut(&request.id) {
        if let Some(n) = request.name {
            asset.name = n;
        }

        if let Some(u_v) = request.unit_value {
            asset.unit_value = u_v;
        }

        Ok(Json(asset.clone()))
    } else {
        Err(routes::Error::NotFound)
    }
}
