use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
};

use crate::graphql::RootQuery;

pub async fn graphql_endpoint(
    schema: Extension<Schema<RootQuery, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}
