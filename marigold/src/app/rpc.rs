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
    palm::{
        accounting::v1 as accounting_v1, babel::v1 as babel_v1, blog::v1 as blog_v1,
        cms::v1 as cms_v1, cups::v1 as cups_v1, forum::v1 as forum_v1, portal::v1 as portal_v1,
    },
    plugins::portal::services::{cups::Server as CupsServerImpl, site::Server as SiteServerImpl},
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

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(cups_v1::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(portal_v1::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(cms_v1::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(blog_v1::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(forum_v1::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(accounting_v1::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(babel_v1::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let addr = format!("0.0.0.0:{}", port);
    log::info!("start gRPC server on http://{}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(cups_v1::cups_server::CupsServer::new(CupsServerImpl {
            redis: redis.clone(),
            jwt: jwt.clone(),
        }))
        .add_service(portal_v1::site_server::SiteServer::new(SiteServerImpl {
            redis,
            jwt,
        }))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // openssl rand -base64 32
    #[serde(rename = "secret-key")]
    secret_key: String,
    // openssl rand -base64 128
    #[serde(rename = "jwt-key")]
    jwt_key: String,
    postgresql: PostgreSql,
    rabbitmq: RabbitMq,
    redis: Redis,
    minio: Minio,
    opensearch: OpenSearch,
}

async fn service_status(server: &Heartbeat, reporter: HealthReporter) {
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        if server.check().await.is_ok() {
            reporter
                .set_serving::<cups_v1::cups_server::CupsServer<CupsServerImpl>>()
                .await;
        } else {
            reporter
                .set_not_serving::<cups_v1::cups_server::CupsServer<CupsServerImpl>>()
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
