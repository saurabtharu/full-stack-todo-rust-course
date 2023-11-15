mod always_errors;
mod get_json;
mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod path_variables;
mod query_params;
mod read_middleware_custom;
mod returns_201;
mod set_middleware_custom_header;
mod validate_with_serde;

use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

use always_errors::always_error;
use get_json::get_json;
use hello_world::hello_world;
use middleware_message::middleware_msg;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use path_variables::{hardcoded_path, path_variables};
use query_params::query_params;
use read_middleware_custom::read_middleware_header;
use returns_201::returns_201;
use set_middleware_custom_header::set_middleware_custom_header;
use validate_with_serde::validate_with_serde;

#[derive(Clone)]
pub struct SharedData {
    pub msg: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        msg: "Hello from CORS headers".to_owned(),
    };

    Router::new()
        .route("/read_middleware_header", get(read_middleware_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/42", get(hardcoded_path))
        // :id -> the placeholder of any variable number
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_msg))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_error", get(always_error))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_user", post(validate_with_serde))
}
