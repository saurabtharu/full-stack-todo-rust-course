pub mod account;
pub mod create_task;
pub mod custom_json_extrator;
pub mod delete_task;
pub mod get_tasks;
pub mod guard;
pub mod hello_world;
pub mod partial_update_task;
pub mod update_tasks;
pub mod validate_with_serde;

use axum::{extract::FromRef, middleware};
pub use axum::{
    routing::{delete, get, patch, post, put},
    Extension, Router,
};

use account::users::{create_user, login_user, logout};
use create_task::create_task;
use custom_json_extrator::custom_json_extrator;
use delete_task::delete_task;
use get_tasks::{get_all_tasks, get_one_task};
use guard::guard;
use hello_world::hello_world;
use partial_update_task::partial_update;
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;
use validate_with_serde::validate_with_serde;

#[derive(Clone, FromRef)]
pub struct AppState {
    database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
        .route("/users/logout", post(logout))
        .route("/hello_world", get(hello_world))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extrator", post(custom_json_extrator))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_task))
        .route("/users", post(create_user))
        .route("/users/login", post(login_user))
        // .layer(Extension(database))
        .with_state(app_state)
}
