use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    message: String,
    sender: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    sender_request: UserRequest,
    message: String,
}

pub async fn custom_json_extrator(Json(msg): Json<UserRequest>) -> Json<UserResponse> {
    Json(UserResponse {
        sender_request: msg,
        message: "Hello back from server".to_owned(),
    })
}
