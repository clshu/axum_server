use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
};

use crate::graphql::GqlSchema;

pub async fn graphql_endpoint(
    schema: Extension<GqlSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}
