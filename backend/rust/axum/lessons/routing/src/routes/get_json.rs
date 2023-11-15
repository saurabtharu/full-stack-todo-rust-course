use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "I am in data".to_owned(),
        count: 4269,
        username: "baruas".to_owned(),
    };
    Json(data)
}
