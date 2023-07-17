mod greeting;

use async_graphql::MergedObject;
use greeting::GreetingQuery;

#[derive(MergedObject, Default)]
pub struct RootQuery(GreetingQuery);
