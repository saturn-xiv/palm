use std::{net::SocketAddr, path::Path, sync::Arc};

use axum::{
    Extension, Router,
    routing::{MethodFilter, get, on},
};
use clap::ValueEnum;
use juniper_axum::{graphiql, playground};
use phlox::{
    Key, Result, cache::redis::Node as Redis, is_stopped, jwt::JwtHS512, minio::Node as Minio,
    orm::postgresql::Node as PostgreSql, parse_toml, queue::rabbitmq::Node as RabbitMq,
};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use tokio::net::TcpListener;

use super::super::{
    graphql::{
        context::Context, handler as graphql_handler, new as new_schema,
        subscriptions as graphql_subscriptions,
    },
    plugins::portal::controllers as portal,
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
    let redis = Arc::new(config.redis.single()?);
    let jwt = Arc::new({
        let key: Key = config.jwt_key.parse()?;
        JwtHS512::new(&key.0)
    });
    let postgresql = config.postgresql.open()?;
    let queue = config.rabbitmq.open().await?;

    let schema = new_schema();
    let ctx = Context {
        db: postgresql,
        cache: redis,
        jwt,
        queue: Arc::new(queue),
    };

    let app = Router::new()
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler),
        )
        .route("/subscriptions", get(graphql_subscriptions))
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .route("/", get(portal::home))
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(ctx));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;
    log::info!("listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "secret-key")]
    secret_key: String,
    #[serde(rename = "jwt-key")]
    jwt_key: String,
    postgresql: PostgreSql,
    rabbitmq: RabbitMq,
    redis: Redis,
    minio: Minio,
    phlox: Rpc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rpc {
    pub host: String,
    pub port: u16,
}
