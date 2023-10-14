mod api;
mod database;
mod views;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    database::sqlite::initialise_sqlite();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello", get(|| views::index::index()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
