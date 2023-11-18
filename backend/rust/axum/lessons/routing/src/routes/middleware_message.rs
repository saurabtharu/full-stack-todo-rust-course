use axum::{extract::State, Extension};

use super::SharedData;

pub async fn middleware_msg(State(message): State<String>) -> String {
    message
}
