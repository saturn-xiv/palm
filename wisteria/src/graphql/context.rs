use std::ops::Deref;
use std::sync::Arc;

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use portal::{
    Dahlia, Loquat, Marigold, cache::redis::StandalonePool as Cache, graphql::Session,
    minio::Client as S3, open_search::Client as Search, orm::postgresql::Pool as Db,
    queue::rabbitmq::Client as RabbitMq,
};

#[derive(Clone)]
pub struct State(pub Arc<InnerState>);

pub struct InnerState {
    pub cookie_key: Key,
    pub cache: Cache,
    pub db: Db,
    pub s3: S3,
    pub queue: RabbitMq,
    pub search: Search,
    pub dahlia: Dahlia,
    pub loquat: Loquat,
    pub marigold: Marigold,
}

impl Deref for State {
    type Target = InnerState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRef<State> for Key {
    fn from_ref(state: &State) -> Self {
        state.0.cookie_key.clone()
    }
}

pub struct Context {
    pub state: Arc<InnerState>,
    pub session: Arc<Session>,
}

impl juniper::Context for Context {}
