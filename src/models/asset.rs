use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
}
