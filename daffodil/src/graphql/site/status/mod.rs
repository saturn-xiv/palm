pub mod minio;
pub mod opensearch;
pub mod postgresql;
pub mod rabbitmq;
pub mod redis;

use std::ops::DerefMut;

use ::opensearch::OpenSearch;
use casbin::Enforcer;
use juniper::GraphQLObject;
use petunia::{
    cache::redis::Pool as CachePool, jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool,
    queue::amqp::RabbitMq, s3::Client as Minio, session::Session, Result,
};
use tokio::sync::Mutex;

use super::super::super::session::current_user;

#[derive(GraphQLObject)]
#[graphql(name = "GetSiteStatusResponse")]
pub struct Response {
    pub opensearch: opensearch::Status,
    pub rabbitmq: rabbitmq::Status,
    pub postgresql: postgresql::Status,
    pub redis: redis::Status,
    pub minio: minio::Status,
}

impl Response {
    pub async fn new(
        ss: &Session,
        (db, ch, queue, minio, search): (&DbPool, &CachePool, &RabbitMq, &Minio, &OpenSearch),
        (jwt, enforcer): (&Jwt, &Mutex<Enforcer>),
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            user.is_administrator(enf)?;
        }
        let mut ch = ch.get()?;
        let ch = ch.deref_mut();

        let queue = queue.connect().await?;

        let it = Self {
            postgresql: postgresql::Status::new(db)?,
            redis: redis::Status::new(ch)?,
            rabbitmq: rabbitmq::Status::new(&queue).await?,
            minio: minio::Status::new(minio).await?,
            opensearch: opensearch::Status::new(search).await?,
        };

        Ok(it)
    }
}
