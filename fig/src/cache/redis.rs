use std::default::Default;
use std::fmt;
use std::fmt::Display;

use ::redis::{Commands, RedisResult, Value, cluster::ClusterClient, cmd};
use chrono::Duration;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::super::Result;

pub type ClusterConnection = ::redis::cluster::ClusterConnection;
pub type Connection = ClusterClient;
pub type Pool = r2d2::Pool<Connection>;
pub type PooledConnection = r2d2::PooledConnection<Connection>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    host: String,
    port: u16,
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "redis://{}:{}", self.host, self.port)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub pool: Option<u32>,
    pub namespace: String,
    pub nodes: Vec<Host>,
}

impl Config {
    pub fn open(&self) -> Result<Pool> {
        let client = ClusterClient::new(
            self.nodes
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        )?;

        let pool = r2d2::Pool::builder()
            .max_size(self.pool.unwrap_or(32))
            .build(client)?;
        Ok(pool)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            nodes: vec![
                Host {
                    host: "127.0.0.1".to_string(),
                    port: 6371,
                },
                Host {
                    host: "127.0.0.1".to_string(),
                    port: 6372,
                },
                Host {
                    host: "127.0.0.1".to_string(),
                    port: 6373,
                },
                Host {
                    host: "127.0.0.1".to_string(),
                    port: 6374,
                },
                Host {
                    host: "127.0.0.1".to_string(),
                    port: 6375,
                },
                Host {
                    host: "127.0.0.1".to_string(),
                    port: 6376,
                },
            ],
            namespace: "demo://".to_string(),
            pool: Some(32),
        }
    }
}

// https://redis.io/commands
impl super::Provider for ClusterConnection {
    fn version(&mut self) -> Result<Vec<(String, String)>> {
        let val: Value = cmd("info").query(self)?;

        let mut items = Vec::new();

        if let Value::Map(ref it) = val {
            for (key, val) in it {
                if let Value::BulkString(key) = key {
                    if let Value::BulkString(val) = val {
                        let key = std::str::from_utf8(key)?;
                        let val = std::str::from_utf8(val)?;
                        items.push((key.to_string(), val.to_string()));
                    }
                }
            }
        }

        Ok(items)
    }
    fn keys(&mut self) -> Result<Vec<(String, i64)>> {
        let mut items = Vec::new();

        let keys: Vec<Value> = Commands::keys(self, "*")?;

        for it in keys.iter() {
            if let Value::BulkString(key) = it {
                let key = std::str::from_utf8(key)?;
                let ttl: i64 = self.ttl(key)?;
                items.push((key.to_string(), ttl));
            }
        }
        Ok(items)
    }

    fn fetch<K, V>(&mut self, key: &K) -> Result<V>
    where
        K: Display,
        V: DeserializeOwned,
    {
        let key = key.to_string();
        let buf: RedisResult<Vec<u8>> = Commands::get(self, &key);
        let it = flexbuffers::from_slice(buf?.as_slice())?;
        Ok(it)
    }
    fn set<K, V>(&mut self, key: &K, val: &V, ttl: Duration) -> Result<()>
    where
        K: Display,
        V: Serialize,
    {
        let key = key.to_string();
        let _: String = self.set_ex(
            &key,
            flexbuffers::to_vec(val)?.as_slice(),
            ttl.num_seconds() as u64,
        )?;
        Ok(())
    }

    fn get<K, V, F>(&mut self, key: &K, fun: F, ttl: Duration) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Display,
        V: DeserializeOwned + Serialize,
    {
        let key = key.to_string();
        let buf: RedisResult<Vec<u8>> = Commands::get(self, &key);
        if let Ok(buf) = buf {
            if let Ok(val) = flexbuffers::from_slice(buf.as_slice()) {
                return Ok(val);
            }
        }
        debug!("cache expire, set {:?} {:?}", key, ttl);
        let val = fun()?;
        let _: String = self.set_ex(
            &key,
            flexbuffers::to_vec(&val)?.as_slice(),
            ttl.num_seconds() as u64,
        )?;
        Ok(val)
    }

    fn clear(&mut self) -> Result<()> {
        let rst = cmd("flushall").query::<String>(self)?;
        info!("{}", rst);
        Ok(())
    }

    fn heartbeat(&mut self) -> Result<()> {
        let rst = cmd("ping").query::<String>(self)?;
        info!("{}", rst);
        Ok(())
    }

    fn destroy<K: Display>(&mut self, key: &K) -> Result<()> {
        warn!("clear cache with prefix {}", key);
        let keys: Vec<String> = Commands::keys(self, format!("{key}*"))?;
        self.del::<_, ()>(&keys)?;
        Ok(())
    }
}
