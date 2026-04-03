use std::{
    ops::{Deref, DerefMut},
    path::Path,
    sync::Arc,
    time::Duration,
};

use phlox::{
    Key, Result,
    cache::redis::{Commands, Node as Redis, SingleClient as RedisClient},
    is_stopped,
    jwt::JwtHS512,
    minio::Node as Minio,
    open_search::Node as OpenSearch,
    orm::postgresql::Node as PostgreSql,
    parse_toml,
    queue::rabbitmq::Node as RabbitMq,
};
use serde::{Deserialize, Serialize};
use tonic::transport::Server;
use tonic_health::server::HealthReporter;

use super::super::{
    palm::cups::v1::cups_server::CupsServer,
    plugins::portal::services::cups::Server as CupsServerImpl,
};

pub async fn start<P: AsRef<Path>>(config: P, port: u16) -> Result<()> {
    if is_stopped() {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;
    let redis = Arc::new(config.redis.single()?);
    let jwt = Arc::new({
        let key: Key = config.jwt_key.parse()?;
        JwtHS512::new(&key.0)
    });

    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    let heartbeat = Arc::new(Heartbeat {
        redis: redis.clone(),
    });
    {
        let heartbeat = heartbeat.clone();

        tokio::spawn(async move {
            let heartbeat = heartbeat.deref();
            service_status(heartbeat, health_reporter.clone()).await;
        });
    }

    let addr = format!("0.0.0.0:{}", port);
    log::info!("start gRPC server on http://{}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(CupsServer::new(CupsServerImpl { redis, jwt }))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    secrets_key: String,
    jwt_key: String,
    postgresql: PostgreSql,
    rabbitmq: RabbitMq,
    redis: Redis,
    minio: Minio,
    open_search: OpenSearch,
}

async fn service_status(server: &Heartbeat, reporter: HealthReporter) {
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        if server.check().await.is_ok() {
            reporter.set_serving::<CupsServer<CupsServerImpl>>().await;
        } else {
            reporter
                .set_not_serving::<CupsServer<CupsServerImpl>>()
                .await;
        }
    }
}

struct Heartbeat {
    redis: Arc<RedisClient>,
}

impl Heartbeat {
    async fn check(&self) -> Result<()> {
        {
            let mut db = self.redis.pool.get()?;
            let db = db.deref_mut();
            let _: String = db.ping()?;
        }
        Ok(())
    }
}
