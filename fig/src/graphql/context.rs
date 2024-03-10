use std::sync::Arc;

use camelia::{models::user::Item as User, orm::postgresql::Pool as PostgreSql};
use palm::{
    cache::redis::Pool as Redis,
    crypto::{aes::Aes, hmac::Hmac},
    jwt::openssl::Jwt,
    minio::Connection as Minio,
    queue::rabbitmq::amqp::Connection as RabbitMq,
    search::Pool as OpenSearch,
};

pub struct Context {
    pub jwt: Arc<Jwt>,
    pub aes: Arc<Aes>,
    pub hmac: Arc<Hmac>,
    pub postgresql: PostgreSql,
    pub redis: Redis,
    pub minio: Arc<Minio>,
    pub rabbitmq: Arc<RabbitMq>,
    pub opensearch: Arc<OpenSearch>,

    pub home: String,
    pub client_ip: String,
    pub token: Option<String>,
    pub user: Option<User>,
    pub lang: String,
}

impl juniper::Context for Context {}
