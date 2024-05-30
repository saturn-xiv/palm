use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration as StdDuration;

use camelia::{orm::postgresql::Config as PostgreSql, services as camelia_services};
use clap::Parser;
use daffodil::services as daffodil_services;
use hibiscus::{
    cache::redis::Config as Redis, parser::from_toml, queue::rabbitmq::Config as RabbitMq,
    search::Config as OpenSearch,
};
use log::{debug, info};
use palm::{camelia::v1 as camelia_v1, daffodil::v1 as daffodil_v1, Result, Thrift};
use serde::{Deserialize, Serialize};
use tonic::transport::Server as TransportServer;
use tonic_health::server::HealthReporter;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Server {
    #[clap(short, long)]
    pub port: u16,
}

impl Server {
    pub async fn launch<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: Config = from_toml(config_file)?;
        let rabbitmq = Arc::new(config.rabbitmq.clone());
        let loquat = Arc::new(config.loquat.clone());
        let gourd = Arc::new(config.gourd.clone());
        let jasmine = Arc::new(config.jasmine.clone());

        let postgresql = config.postgresql.open()?;
        let redis = config.redis.open()?;
        let opensearch = config.opensearch.open()?;
        {
            debug!("{:?}", opensearch.info().await?);
        }
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), self.port);
        info!("start gRPC at {}", addr);
        let (health_reporter, health_service) = tonic_health::server::health_reporter();
        info!("start services health status reporter");
        tokio::spawn(services_status(health_reporter));
        TransportServer::builder()
            .add_service(camelia_v1::user_server::UserServer::new(
                camelia_services::user::Service {
                    loquat: loquat.clone(),
                    gourd: gourd.clone(),
                    queue: rabbitmq.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(camelia_v1::attachment_server::AttachmentServer::new(
                camelia_services::attachment::Service {
                    loquat: loquat.clone(),
                    gourd: gourd.clone(),
                    jasmine: jasmine.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(camelia_v1::leave_word_server::LeaveWordServer::new(
                camelia_services::leave_word::Service {
                    loquat: loquat.clone(),
                    gourd: gourd.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(camelia_v1::locale_server::LocaleServer::new(
                camelia_services::locale::Service {
                    loquat: loquat.clone(),
                    gourd: gourd.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(camelia_v1::site_server::SiteServer::new(
                camelia_services::site::Service {
                    loquat: loquat.clone(),
                    gourd: gourd.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(daffodil_v1::book_server::BookServer::new(
                daffodil_services::book::Service {
                    loquat: loquat.clone(),
                    jasmine: jasmine.clone(),
                    gourd: gourd.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(daffodil_v1::account_server::AccountServer::new(
                daffodil_services::account::Service {
                    loquat: loquat.clone(),
                    jasmine: jasmine.clone(),
                    gourd: gourd.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(daffodil_v1::merchant_server::MerchantServer::new(
                daffodil_services::merchant::Service {
                    loquat: loquat.clone(),
                    jasmine: jasmine.clone(),
                    gourd: gourd.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(daffodil_v1::transaction_server::TransactionServer::new(
                daffodil_services::transaction::Service {
                    loquat: loquat.clone(),
                    jasmine: jasmine.clone(),
                    gourd: gourd.clone(),
                    db: postgresql.clone(),
                    cache: redis.clone(),
                },
            ))
            .add_service(health_service)
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Config {
    loquat: Thrift,
    gourd: Thrift,
    jasmine: Thrift,
    postgresql: PostgreSql,
    redis: Redis,
    rabbitmq: RabbitMq,
    opensearch: OpenSearch,
}

async fn services_status(mut reporter: HealthReporter) {
    loop {
        reporter
            .set_serving::<camelia_v1::user_server::UserServer<camelia_services::user::Service>>()
            .await;
        reporter
        .set_serving::<camelia_v1::attachment_server::AttachmentServer<camelia_services::attachment::Service>>()
        .await;
        reporter
            .set_serving::<camelia_v1::leave_word_server::LeaveWordServer<camelia_services::leave_word::Service>>()
            .await;
        reporter
            .set_serving::<camelia_v1::locale_server::LocaleServer<camelia_services::locale::Service>>()
            .await;
        reporter
            .set_serving::<camelia_v1::site_server::SiteServer<camelia_services::site::Service>>()
            .await;
        reporter
            .set_serving::<daffodil_v1::book_server::BookServer<daffodil_services::book::Service>>()
            .await;
        reporter
            .set_serving::<daffodil_v1::account_server::AccountServer<daffodil_services::account::Service>>()
            .await;
        reporter
            .set_serving::<daffodil_v1::merchant_server::MerchantServer<daffodil_services::merchant::Service>>()
            .await;
        reporter
            .set_serving::<daffodil_v1::transaction_server::TransactionServer<
                daffodil_services::transaction::Service,
            >>()
            .await;
        tokio::time::sleep(StdDuration::from_secs(3)).await;
    }
}
