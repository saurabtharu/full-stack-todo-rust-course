mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod path_variables;
mod query_params;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use path_variables::{hardcoded_path, path_variables};
use query_params::query_params;

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/42", get(hardcoded_path))
        // :id -> the placeholder of any variable number
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .layer(cors)
}
