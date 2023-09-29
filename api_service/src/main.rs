mod graphql;
mod services;

use async_graphql::{Schema,EmptySubscription, Request, http::{playground_source, GraphQLPlaygroundConfig}};
use async_graphql_warp::GraphQLResponse;
use crate::graphql::*;
use std::convert::Infallible;
use warp::{Filter, filters::BoxedFilter, Reply, http::Response};

extern crate log;
extern crate pretty_env_logger;

fn make_routes() -> BoxedFilter<(impl Reply,)> {
    let schema = Schema::build(Query, Mutation, EmptySubscription)
    .finish();

    let graphql_handler = warp::post().and(warp::path("graphql"))
    .and(
        async_graphql_warp::graphql(schema)
        .and_then(|(schema, request): (Schema<_,_,_>, Request) | async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        })
    );

    let graphql_playground = warp::path("playground").map(|| {
        Response::builder()
        .header("content-type", "text/html")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    graphql_handler
    .or(graphql_playground)
    .boxed()
}

#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    warp::serve(make_routes()).run(([127, 0, 0, 1], 8000)).await;
}
