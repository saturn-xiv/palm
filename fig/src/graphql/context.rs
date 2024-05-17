use std::sync::Arc;

use camelia::{
    controllers::{Gourd, Jasmine, Loquat},
    orm::postgresql::Pool as PostgreSql,
};
use hibiscus::{
    cache::redis::Pool as Redis, queue::rabbitmq::Config as RabbitMq, search::Config as OpenSearch,
    session::Session,
};

pub struct Context {
    pub loquat: Arc<Loquat>,
    pub gourd: Arc<Gourd>,
    pub jasmine: Arc<Jasmine>,
    pub postgresql: PostgreSql,
    pub redis: Redis,
    pub rabbitmq: Arc<RabbitMq>,
    pub opensearch: Arc<OpenSearch>,

    pub home: String,
    pub session: Session,
}

impl juniper::Context for Context {}
