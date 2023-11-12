use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
    name: String,
    age: u8,
}

#[derive(Debug, Serialize)]
pub struct MirrorJsonResponse {
    message_from_client: MirrorJson,
    message_from_server: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message_from_client: body,
        message_from_server: "Hello from axum server".to_owned(),
    })
}
