use axum::{routing::get, Router};
pub async fn run() {
    // build our applicaiton with a single route
    let app = Router::new().route("/", get(hello_world));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> String {
    "Hello world!!!!!".to_string()
}
