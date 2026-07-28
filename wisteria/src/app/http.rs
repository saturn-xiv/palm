use std::path::Path;

use clap::ValueEnum;
use portal::{
    Key, Result, cache::redis::Node as Redis, is_stopped, minio::Node as Minio,
    open_search::Node as OpenSearch, orm::postgresql::Node as PostgreSql, parse_toml,
    queue::rabbitmq::Node as RabbitMq,
};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

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

pub async fn start<P: AsRef<Path>>(config: P, _port: u16, _theme: Theme) -> Result<()> {
    if is_stopped()? {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let _config: Config = parse_toml(config)?;
    // let redis = Arc::new(config.redis.single()?);
    // let jwt = Arc::new({
    //     let key: Key = config.jwt_key.parse()?;
    //     JwtHS512::new(&key.0)
    // });
    // let postgresql = config.postgresql.open()?;
    // let queue = config.rabbitmq.open().await?;

    // let schema = new_schema();
    // let ctx = Context {
    //     db: postgresql,
    //     cache: redis,
    //     jwt,
    //     queue: Arc::new(queue),
    // };

    // let app = Router::new()
    //     .route(
    //         "/graphql",
    //         on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler),
    //     )
    //     .route("/subscriptions", get(graphql_subscriptions))
    //     .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
    //     .route("/playground", get(playground("/graphql", "/subscriptions")))
    //     .route("/", get(portal::home))
    //     .layer(Extension(Arc::new(schema)))
    //     .layer(Extension(ctx));

    // let addr = SocketAddr::from(([127, 0, 0, 1], port));
    // let listener = TcpListener::bind(addr).await?;
    // log::info!("listening on {addr}");
    // axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "cookie-key")]
    cookie_key: Key,
    postgresql: PostgreSql,
    redis: Redis,
    rabbitmq: RabbitMq,
    minio: Minio,
    opensearch: OpenSearch,
    loquat: Rpc,
    dahlia: Rpc,
    marigold: Rpc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rpc {
    pub host: String,
    pub port: u16,
}
