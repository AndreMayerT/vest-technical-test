mod mutation;
mod query;
mod types;

pub use mutation::Mutation;
pub use query::Query;
pub use types::*;
use async_graphql::*;
use reqwest::Client;

pub fn create_schema() -> Schema<Query, Mutation, EmptySubscription> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()
        .expect("Failed to build reqwest client");

    Schema::build(Query, Mutation, EmptySubscription)
        .data(client)
        .finish()
}