pub mod handlers;
pub mod resolvers;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use resolvers::Query;

pub type GqlSchema = Schema<Query, EmptyMutation, EmptySubscription>;
