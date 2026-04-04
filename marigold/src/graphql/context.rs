use std::sync::Arc;

use phlox::{
    cache::redis::SingleClient as Redis, jwt::JwtHS512 as Jwt, orm::postgresql::Pool as PostgreSql,
    queue::rabbitmq::Client as RabbitMq,
};

#[derive(Clone)]
pub struct Context {
    pub db: PostgreSql,
    pub cache: Arc<Redis>,
    pub queue: Arc<RabbitMq>,
    pub jwt: Arc<Jwt>,
}

impl juniper::Context for Context {}
