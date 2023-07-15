// #![allow(unused_imports)]
use axum::extract::{Json, Path, Query};
use axum::response::IntoResponse;
use axum::{routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // build our application with a single route
    let app = Router::new()
        .route("/foo", get(get_foo))
        .route("/bar/:user_id", get(get_bar))
        .route("/user/login", post(login))
        .route("/", get(root));

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

#[derive(Deserialize, Serialize, Debug)]
struct UserParams {
    name: String,
    age: u8,
}

// `Query` gives you the query parameters and deserializes them.
// async fn query(Query(params): Query<HashMap<String, String>>) {}
async fn get_foo(Query(params): Query<UserParams>) -> impl IntoResponse {
    Json(json!(params))
}

// `Path` gives you the path parameters and deserializes them.
// async fn path(Path(user_id): Path<u32>) {}

async fn get_bar(Path(user_size): Path<u32>) -> impl IntoResponse {
    Json(json!({ "user_size": user_size }))
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginParams {
    username: String,
    password: String,
}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
// async fn json(Json(payload): Json<serde_json::Value>) {}

async fn login(Json(params): Json<LoginParams>) -> impl IntoResponse {
    Json(json!(params))
}
