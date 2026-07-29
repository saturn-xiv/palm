use std::ops::{Deref, DerefMut};
use std::result::Result as StdResult;
use std::sync::Arc;
use std::time::Duration;

use hyacinth::{FlexbufferReader, FlexbufferSerializer, ProtobufMessage};
use r2d2::{
    Error as R2d2Error, ManageConnection, Pool as R2d2Pool,
    PooledConnection as R2d2PooledConnection,
};
use redis::{
    Client as RedisClient, Commands, Connection as RedisConnection, RedisError, RedisResult,
    cluster::{ClusterClient as RedisClusterClient, ClusterConnection as RedisClusterConnection},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::super::Result;

pub type ClusterPool = Pool<RedisClusterConnection, RedisClusterClient>;
pub type ClusterConnection = PooledConnection<RedisClusterConnection, RedisClusterClient>;
pub type StandalonePool = Pool<RedisConnection, RedisClient>;
pub type StandaloneConnection = PooledConnection<RedisConnection, RedisClient>;

fn set<D: Commands>(db: &mut D, key: &str, value: &[u8], ttl: Option<Duration>) -> RedisResult<()> {
    let _: () = match ttl {
        Some(ttl) => db.set_ex(key, value, ttl.as_secs())?,
        None => db.set(key, value)?,
    };
    Ok(())
}

fn get<D: Commands>(db: &mut D, key: &str) -> RedisResult<Vec<u8>> {
    let tmp: Vec<u8> = db.get(key)?;
    Ok(tmp)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(default = "node_default_host")]
    pub host: String,
    #[serde(default = "node_default_port")]
    pub port: u16,
    #[serde(default)]
    pub namespace: Option<String>,
    #[serde(rename = "pool-size", default = "node_default_pool_size")]
    pub pool_size: usize,
}

fn node_default_host() -> String {
    "127.0.0.1".to_string()
}

fn node_default_port() -> u16 {
    6379
}

fn node_default_pool_size() -> usize {
    32
}
impl Default for Node {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6379,
            namespace: None,
            pool_size: 32,
        }
    }
}

impl Node {
    pub fn standalone(&self) -> Result<StandalonePool> {
        log::debug!("open redis host tcp://{}:{}", self.host, self.port);
        let client = RedisClient::open(self.url())?;
        let pool = R2d2Pool::builder()
            .max_size(self.pool_size as u32)
            .build(client)?;
        Ok(Pool {
            pool,
            namespace: Arc::new(self.namespace.clone()),
        })
    }
    pub fn cluster(&self) -> Result<ClusterPool> {
        log::debug!("open redis cluster tcp://{}:{}", self.host, self.port);
        let client = RedisClusterClient::new(vec![self.url()])?;
        let pool = R2d2Pool::builder()
            .max_size(self.pool_size as u32)
            .build(client)?;
        Ok(Pool {
            pool,
            namespace: Arc::new(self.namespace.clone()),
        })
    }

    fn url(&self) -> String {
        format!("redis://{}:{}/", self.host, self.port)
    }
}

pub struct Pool<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> {
    namespace: Arc<Option<String>>,
    pool: R2d2Pool<T>,
}

impl<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> Pool<C, T> {
    pub fn get(&self) -> StdResult<PooledConnection<C, T>, R2d2Error> {
        let connection = self.pool.get()?;
        Ok(PooledConnection {
            namespace: self.namespace.clone(),
            connection,
        })
    }
}

pub struct PooledConnection<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> {
    namespace: Arc<Option<String>>,
    connection: R2d2PooledConnection<T>,
}

impl<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> PooledConnection<C, T> {
    fn key<S: AsRef<str>>(&self, k: S) -> String {
        let ns = self.namespace.deref();
        match ns {
            Some(s) => format!("{}://{}", s, k.as_ref()),
            None => k.as_ref().to_string(),
        }
    }
}

impl<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> super::ProtobufCacher
    for PooledConnection<C, T>
{
    fn set<K: AsRef<str>, V: ProtobufMessage>(
        &mut self,
        key: K,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let buf = value.serialize()?;
        let key = self.key(key);
        let db = self.connection.deref_mut();
        set(db, &key, &buf, ttl)?;
        Ok(())
    }
    fn get<K: AsRef<str>, V: ProtobufMessage + Default>(&mut self, key: K) -> Result<V> {
        let key = self.key(key);
        let db = self.connection.deref_mut();
        let buf = get(db, &key)?;
        let it = V::parse(&buf[..])?;
        Ok(it)
    }
}

impl<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> super::FlexBuffersCacher
    for PooledConnection<C, T>
{
    fn set<K: AsRef<str>, V: Serialize>(
        &mut self,
        key: K,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let mut se = FlexbufferSerializer::new();
        value.serialize(&mut se)?;
        let buf = se.view();
        let key = self.key(key);
        let db = self.connection.deref_mut();
        set(db, &key, buf, ttl)?;
        Ok(())
    }
    fn get<K: AsRef<str>, V: DeserializeOwned>(&mut self, key: K) -> Result<V> {
        let key = self.key(key);
        let db = self.connection.deref_mut();
        let buf = get(db, &key)?;
        let reader = FlexbufferReader::get_root(&buf[..])?;
        let it = V::deserialize(reader)?;
        Ok(it)
    }
}
