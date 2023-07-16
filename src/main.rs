// #![allow(unused_imports)]
mod graphql;
mod utils;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{Extension, Json, Path, Query},
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;

use graphql::greeting::RootQuery;
use utils::constants::GRAPHQL_PATH;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let schema = Schema::build(RootQuery, EmptyMutation, EmptySubscription).finish();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // build our application with a single route
    let app = Router::new()
        .route("/foo", get(get_foo))
        .route("/bar/:user_id", get(get_bar))
        .route("/user/login", post(login))
        .route("/", get(root))
        .route(GRAPHQL_PATH, get(graphql_playground).post(graphql_endpoint))
        .layer(Extension(schema));

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

async fn graphql_endpoint(
    schema: Extension<Schema<RootQuery, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}
