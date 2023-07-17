pub mod handlers;
pub mod resolvers;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use resolvers::RootQuery;

pub type GqlSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;
