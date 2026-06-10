use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAsset {
    pub name: String,
    pub unit_value: f64,
}

#[derive(Deserialize)]
pub struct UpdateAsset {
    pub id: i64,
    pub name: Option<String>,
    pub unit_value: Option<f64>,
}
