use std::sync::Arc;

use portal::{
    Dahlia, Loquat, Marigold, cache::redis::StandaloneClient as Cache, graphql::Session,
    minio::Client as S3, open_search::Client as Search, orm::postgresql::Pool as Db,
    queue::rabbitmq::Client as RabbitMq,
};

pub struct State {
    pub cache: Cache,
    pub db: Db,
    pub s3: S3,
    pub queue: RabbitMq,
    pub search: Search,
    pub dahlia: Dahlia,
    pub loquat: Loquat,
    pub marigold: Marigold,
}

pub struct Context {
    pub state: Arc<State>,
    pub session: Arc<Session>,
}

impl juniper::Context for Context {}
