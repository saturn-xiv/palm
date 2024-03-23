use std::sync::Arc;

use camelia::orm::postgresql::Pool as PostgreSql;
use casbin::Enforcer;
use palm::{
    cache::redis::Pool as Redis,
    crypto::{aes::Aes, hmac::Hmac},
    jwt::openssl::Jwt,
    minio::Client as Minio,
    queue::rabbitmq::amqp::Connection as RabbitMq,
    search::Pool as OpenSearch,
    session::Session,
};
use tokio::sync::Mutex;

pub struct Context {
    pub jwt: Arc<Jwt>,
    pub aes: Arc<Aes>,
    pub hmac: Arc<Hmac>,
    pub postgresql: PostgreSql,
    pub redis: Redis,
    pub minio: Arc<Minio>,
    pub rabbitmq: Arc<RabbitMq>,
    pub opensearch: Arc<OpenSearch>,
    pub enforcer: Arc<Mutex<Enforcer>>,

    pub home: String,
    pub session: Session,
}

impl juniper::Context for Context {}
