use std::{ops::DerefMut, time::Duration};

use flexbuffers::{FlexbufferSerializer, Reader as FlexbufferReader};
use prost::Message as ProtobufMessage_;
use r2d2::Pool;
use redis::{
    Client as RedisClient, Connection as RedisConnection, RedisResult,
    cluster::{ClusterClient as RedisClusterClient, ClusterConnection as RedisClusterConnection},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub use r2d2::ManageConnection;
pub use redis::{Commands, RedisError};

use super::super::Result;

pub type ClusterClient = Client<RedisClusterConnection, RedisClusterClient>;
pub type SingleClient = Client<RedisConnection, RedisClient>;

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
    pub fn single(&self) -> Result<SingleClient> {
        log::info!("open redis host tcp://{}:{}", self.host, self.port);
        let client = RedisClient::open(self.url())?;
        let pool = Pool::builder()
            .max_size(self.pool_size as u32)
            .build(client)?;
        Ok(Client {
            pool,
            namespace: self.namespace.clone(),
        })
    }
    pub fn cluster(&self) -> Result<ClusterClient> {
        log::info!("open redis cluster tcp://{}:{}", self.host, self.port);
        let client = RedisClusterClient::new(vec![self.url()])?;
        let pool = Pool::builder()
            .max_size(self.pool_size as u32)
            .build(client)?;
        Ok(Client {
            pool,
            namespace: self.namespace.clone(),
        })
    }

    fn url(&self) -> String {
        format!("redis://{}:{}/", self.host, self.port)
    }
}

pub struct Client<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> {
    namespace: Option<String>,
    pub pool: Pool<T>,
}

impl<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> Client<C, T> {
    fn key<S: AsRef<str>>(&self, k: S) -> String {
        match self.namespace {
            Some(ref s) => format!("{}://{}", s, k.as_ref()),
            None => k.as_ref().to_string(),
        }
    }
}

impl<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> super::ProtobufCacher
    for Client<C, T>
{
    fn set<K: AsRef<str>, V: ProtobufMessage_>(
        &self,
        key: K,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let mut buf = Vec::new();
        value.encode(&mut buf)?;
        let key = self.key(key);
        let mut db = self.pool.get()?;
        let db = db.deref_mut();
        set(db, &key, &buf, ttl)?;
        Ok(())
    }
    fn get<K: AsRef<str>, V: ProtobufMessage_ + Default>(&self, key: K) -> Result<V> {
        let key = self.key(key);
        let mut db = self.pool.get()?;
        let db = db.deref_mut();
        let buf = get(db, &key)?;
        let it = V::decode(&buf[..])?;
        Ok(it)
    }
}

impl<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>> super::FlexBuffersCacher
    for Client<C, T>
{
    fn set<K: AsRef<str>, V: Serialize>(
        &self,
        key: K,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let mut se = FlexbufferSerializer::new();
        value.serialize(&mut se)?;
        let buf = se.view();
        let key = self.key(key);
        let mut db = self.pool.get()?;
        let db = db.deref_mut();
        set(db, &key, buf, ttl)?;
        Ok(())
    }
    fn get<K: AsRef<str>, V: DeserializeOwned>(&self, key: K) -> Result<V> {
        let key = self.key(key);
        let mut db = self.pool.get()?;
        let db = db.deref_mut();
        let buf = get(db, &key)?;
        let reader = FlexbufferReader::get_root(&buf[..])?;
        let it = V::deserialize(reader)?;
        Ok(it)
    }
}
