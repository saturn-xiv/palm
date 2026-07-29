use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use axum::{
    Extension, Router,
    routing::{MethodFilter, get, on},
};
use axum_extra::extract::cookie::Key as CookieKey;
use clap::ValueEnum;
use hyacinth::{GrpcClientChannel, open_grpc_channel};
use juniper_axum::{graphiql, playground};
use portal::{
    Dahlia, Key, Loquat, Marigold, Result, cache::redis::Node as Redis, is_stopped,
    minio::Node as Minio, open_search::Node as OpenSearch, orm::postgresql::Node as PostgreSql,
    parse_toml, queue::rabbitmq::Node as RabbitMq,
};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use tokio::{net::TcpListener, signal};

use super::super::{
    controllers,
    graphql::{
        context::{InnerState, State},
        handler as graphql_handler, new as new_schema, subscriptions as graphql_subscriptions,
    },
};

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumDisplay,
    EnumString,
    ValueEnum,
    Default,
)]
pub enum Theme {
    #[strum(serialize = "bulma")]
    Bulma,
    #[default]
    #[strum(serialize = "bootstrap")]
    Bootstrap,
}

pub async fn start<P: AsRef<Path>>(config: P, port: u16, _theme: Theme) -> Result<()> {
    if is_stopped()? {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;

    let schema = new_schema();
    let state = State(Arc::new(InnerState {
        cookie_key: {
            let it: Key = config.cookie_key.parse()?;
            CookieKey::from(&it.0)
        },
        db: config.postgresql.open()?,
        cache: config.redis.standalone()?,
        s3: config.minio.open()?,
        search: config.opensearch.single()?,
        loquat: Loquat::new(config.loquat.open()),
        dahlia: Dahlia::new(config.dahlia.open()),
        marigold: Marigold::new(config.marigold.open()),
        queue: config.rabbitmq.open().await?,
    }));

    let app = Router::new()
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler),
        )
        .route("/subscriptions", get(graphql_subscriptions))
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .route("/", get(controllers::home))
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(state.clone()))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;
    log::info!("listening on {addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "cookie-key")]
    cookie_key: String,
    postgresql: PostgreSql,
    redis: Redis,
    rabbitmq: RabbitMq,
    minio: Minio,
    opensearch: OpenSearch,
    loquat: Rpc,
    dahlia: Rpc,
    marigold: Rpc,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rpc {
    #[serde(default = "rpc_default_host")]
    pub host: String,
    pub port: u16,
}

fn rpc_default_host() -> String {
    "localhost".to_string()
}

impl Rpc {
    pub fn open(&self) -> GrpcClientChannel {
        open_grpc_channel(&self.host, self.port)
    }
}
