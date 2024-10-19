use std::sync::Arc;

use casbin::Enforcer;
use opensearch::OpenSearch;
use petunia::{
    cache::redis::Pool as Redis, crypto::Key, jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as PostgreSql, queue::amqp::RabbitMq, s3::Client as Minio,
    session::Session,
};
use tokio::sync::Mutex;

pub struct Context {
    pub secrets: Arc<Key>,
    pub enforcer: Arc<Mutex<Enforcer>>,
    pub rabbitmq: Arc<RabbitMq>,
    pub postgresql: Arc<PostgreSql>,
    pub redis: Arc<Redis>,
    pub session: Session,
    pub jwt: Arc<Jwt>,
    pub search: Arc<OpenSearch>,
    pub minio: Arc<Minio>,
}

impl juniper::Context for Context {}
