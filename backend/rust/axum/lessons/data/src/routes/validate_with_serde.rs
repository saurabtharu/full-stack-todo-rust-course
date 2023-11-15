use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UsernameData {
    username: String,
    password: String,
}

pub async fn validate_with_serde(Json(user): Json<UsernameData>) -> Json<UsernameData> {
    Json(user)
}
