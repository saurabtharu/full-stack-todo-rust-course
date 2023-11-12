mod routes;

// use axum::{routing::get, Router};
use routes::create_routes;

pub async fn run() {
    // let app = Router::new().route("/", get(|| async { "Hello world!!" }));
    let app = create_routes();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
