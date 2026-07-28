pub mod context;
pub mod mutation;
pub mod query;
pub mod subscription;

use std::{ops::Deref, sync::Arc};

use axum::{Extension, extract::WebSocketUpgrade, http::HeaderMap, response::Response};
use juniper::RootNode;
use juniper_axum::{
    extract::JuniperRequest, response::JuniperResponse, subscriptions::serve_graphql_transport_ws,
};
use juniper_graphql_ws::ConnectionConfig;
use portal::graphql::Session;

pub type Schema = RootNode<query::Query, mutation::Mutation, subscription::Subscription>;

pub fn new() -> Schema {
    Schema::new(
        query::Query {},
        mutation::Mutation {},
        subscription::Subscription {},
    )
}

pub async fn handler(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(state): Extension<Arc<context::State>>,
    headers: HeaderMap,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let schema = schema.deref();
    JuniperResponse(
        request
            .execute(
                schema,
                &context::Context {
                    state,
                    session: Arc::new(Session::new(&headers)),
                },
            )
            .await,
    )
}

pub async fn subscriptions(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(state): Extension<Arc<context::State>>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Response {
    ws.protocols(["graphql-transport-ws"])
        .max_frame_size(1024)
        .max_message_size(1024)
        .max_write_buffer_size(100)
        .on_upgrade(move |socket| {
            serve_graphql_transport_ws(
                socket,
                schema,
                ConnectionConfig::new(context::Context {
                    state,
                    session: Arc::new(Session::new(&headers)),
                }),
            )
        })
}
