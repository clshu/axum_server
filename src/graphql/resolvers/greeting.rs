use async_graphql::*;

#[derive(Default)]
pub struct GreetingQuery;

#[Object]
impl GreetingQuery {
    pub async fn greeting(&self) -> &'static str {
        "Greetings, Earthling!"
    }
}
