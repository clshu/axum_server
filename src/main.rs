// #![allow(unused_imports)]
mod graphql;
mod utils;

use async_graphql::{extensions::Logger, EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router};
use dotenv::dotenv;
use http::Method;
use std::env;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeFile,
};

use std::net::SocketAddr;

use graphql::{
    handlers::{graphql_endpoint, graphql_playground},
    resolvers::Query,
};
use utils::constants::GRAPHQL_PATH;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Serve the index.html file from the public folder
    let serve_file = ServeFile::new("./public/index.html");
    // Build a schema
    let schema = if is_axum_env("log") {
        // Enable the logger extension
        Schema::build(Query::default(), EmptyMutation, EmptySubscription)
            .extension(Logger)
            .finish()
    } else {
        Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish()
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route(GRAPHQL_PATH, get(graphql_playground).post(graphql_endpoint))
        .layer(Extension(schema))
        .layer(cors)
        .fallback_service(serve_file);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Test if the environment AXUM_ENV is set to the given value
fn is_axum_env(value: &str) -> bool {
    match env::var("AXUM_ENV") {
        Ok(val) => val == value,
        Err(_) => false,
    }
}
