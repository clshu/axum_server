use async_graphql::*;

pub struct RootQuery;

#[Object]
impl RootQuery {
    pub async fn greeting(&self) -> &'static str {
        "Greetings, Earthling!"
    }
}
