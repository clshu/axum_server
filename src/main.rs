// #![allow(unused_imports)]
mod graphql;
mod utils;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use axum::{extract::Extension, routing::get, Router};

use std::net::SocketAddr;

use graphql::{
    greeting::RootQuery,
    handlers::{graphql_endpoint, graphql_playground},
};
use utils::constants::GRAPHQL_PATH;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let schema = Schema::build(RootQuery, EmptyMutation, EmptySubscription).finish();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // build our application with a single route
    let app = Router::new()
        .route(GRAPHQL_PATH, get(graphql_playground).post(graphql_endpoint))
        .layer(Extension(schema));

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
