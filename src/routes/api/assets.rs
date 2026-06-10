use axum::{Json, Router, routing::get};

use crate::{
    app::AppState,
    auth::Admin,
    dto::{CreateAsset, UpdateAsset},
    error::AppError,
    models::Asset,
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list).post(create).patch(update))
}

#[tracing::instrument(skip_all)]
pub async fn list(repo: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repo.list_assets().await?;
    Ok(Json(assets))
}

#[tracing::instrument(skip_all)]
pub async fn create(
    _admin: Admin,
    repo: Repository,
    Json(request): Json<CreateAsset>,
) -> Result<Json<Asset>, AppError> {
    let new_asset = repo.create_asset(request.name, request.unit_value).await?;

    Ok(Json(new_asset))
}

#[tracing::instrument(skip_all)]
pub async fn update(
    _admin: Admin,
    repo: Repository,
    Json(request): Json<UpdateAsset>,
) -> Result<Json<Asset>, AppError> {
    match repo
        .update_asset(request.id, request.name, request.unit_value)
        .await?
    {
        Some(updated_asset) => Ok(Json(updated_asset)),
        None => Err(AppError::AssetDoesNotExists),
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn test_create_asset(pool: PgPool) {
        let request = CreateAsset {
            name: "Bitcoin".to_string(),
            unit_value: 10.0,
        };
        let Json(new_asset) = create(Admin, pool.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Bitcoin");
        assert_eq!(new_asset.unit_value, 10.0);

        insta::assert_json_snapshot!(new_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_list_assets(pool: PgPool) {
        let Json(assets) = list(pool.into()).await.expect("success");

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Bitcoin");
        assert_eq!(assets[0].unit_value, 10.0);

        insta::assert_json_snapshot!(assets);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_update_asset(pool: PgPool) {
        let request = UpdateAsset {
            id: 1,
            name: Some("Ethereum".to_string()),
            unit_value: Some(20.0),
        };
        let Json(updated_asset) = update(Admin, pool.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(updated_asset.id, 1);
        assert_eq!(updated_asset.name, "Ethereum");
        assert_eq!(updated_asset.unit_value, 20.0);

        insta::assert_json_snapshot!(updated_asset);
    }
}
