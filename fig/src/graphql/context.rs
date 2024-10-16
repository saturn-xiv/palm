use std::sync::Arc;

use opensearch::OpenSearch;
use petunia::{
    cache::redis::Pool as Redis, jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as PostgreSql,
    queue::amqp::RabbitMq, session::Session,
};

pub struct Context {
    pub rabbitmq: Arc<RabbitMq>,
    pub postgresql: Arc<PostgreSql>,
    pub redis: Arc<Redis>,
    pub session: Session,
    pub jwt: Arc<Jwt>,
    pub search: Arc<OpenSearch>,
}

impl juniper::Context for Context {}
