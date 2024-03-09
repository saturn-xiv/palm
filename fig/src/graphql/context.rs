use std::sync::Arc;

use camelia::orm::postgresql::PooledConnection as PostgreSql;
use palm::{
    cache::redis::Pool as Redis, minio::Connection as Minio,
    queue::rabbitmq::amqp::Connection as RabbitMq,
};

pub struct Context {
    pub postgresql: PostgreSql,
    pub redis: Redis,
    pub minio: Arc<Minio>,
    pub rabbitmq: Arc<RabbitMq>,
}

impl juniper::Context for Context {}
