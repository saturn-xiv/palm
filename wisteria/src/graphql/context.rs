use portal::{
    cache::redis::StandaloneClient as Cache, minio::Client as S3, open_search::Client as Search,
    orm::postgresql::Pool as Db, queue::rabbitmq::Client as RabbitMq,
};

pub struct Context {
    pub cache: Cache,
    pub db: Db,
    pub s3: S3,
    pub queue: RabbitMq,
    pub search: Search,
}
