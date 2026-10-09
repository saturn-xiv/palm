pub mod opensearch;
pub mod postgresql;
pub mod rabbitmq;
pub mod seaweedfs;

use std::sync::OnceLock;

use chrono::{NaiveDateTime, Utc};
use juniper::GraphQLObject;

use super::super::super::{
    Jwt, Result, cache::redis::StandaloneConnection as Cache, graphql::Session,
    opensearch::Client as OpenSearch, orm::postgresql::Connection as Db,
    queue::rabbitmq::Client as RabbitMq, rbac::Rbac, s3::seaweedfs::Client as SeaweedFsClient,
};

#[derive(Debug, Default, GraphQLObject)]
#[graphql(name = "SiteStatus")]
pub struct Item {
    pub rabbitmq: rabbitmq::Item,
    pub postgresql: postgresql::Item,
    pub redis: String,
    pub seaweedfs: seaweedfs::Item,
    pub opensearch: opensearch::Item,
    pub client_ip: Option<String>,

    pub build_time: String,
    pub version: String,
    pub launched_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub async fn new<R: Rbac, J: Jwt>(
        ss: &Session,
        (db, cache, queue, search, s3): (
            &mut Db,
            &mut Cache,
            &RabbitMq,
            &OpenSearch,
            &SeaweedFsClient,
        ),
        (rbac, jwt): (&R, &J),
        (version, build_time): (&str, &str),
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        rbac.is_administrator(current_user.id()).await?;
        let it = Self {
            postgresql: postgresql::Item::new(db)?,
            rabbitmq: rabbitmq::Item::new(queue),
            redis: cache.info()?,
            seaweedfs: seaweedfs::Item::new(s3).await?,
            opensearch: opensearch::Item::new(search).await?,
            client_ip: ss.client_ip.clone(),
            created_at: Utc::now().naive_utc(),
            version: version.to_string(),
            build_time: build_time.to_string(),
            launched_at: Self::launched_at(),
        };
        Ok(it)
    }
}

impl Item {
    pub fn launched_at() -> NaiveDateTime {
        *LAUNCHED_AT.get_or_init(|| Utc::now().naive_utc())
    }
}

static LAUNCHED_AT: OnceLock<NaiveDateTime> = OnceLock::new();
