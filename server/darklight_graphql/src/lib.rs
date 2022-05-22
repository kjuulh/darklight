mod darklight;

use std::sync::Arc;
use async_graphql::{EmptyMutation, EmptySubscription, Request, Response, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use axum::{Extension, Json, Router};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use darklight_app::download_queue::DownloadQueue;
use darklight_events::subscriber::subscriber::Subscriber;
use crate::darklight::{DarklightSchema, MutationRoot, QueryRoot, SubscriptionRoot};

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws")))
}

async fn graphql_handler(schema: Extension<DarklightSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

pub struct GraphQLDependencies {
    subscriber: Arc<Subscriber>,
    download_queue: Arc<DownloadQueue>,
}

impl GraphQLDependencies {
    pub fn new(
        subscriber: Arc<Subscriber>,
        download_queue: Arc<DownloadQueue>,
    ) -> Self {
        Self { subscriber, download_queue }
    }
}

pub async fn run(deps: GraphQLDependencies) {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(deps)
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema));

    axum::Server::bind(&"0.0.0.0:8001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}