pub mod redis;

use std::fmt::Display;
use std::time::Duration;

use serde::{de::DeserializeOwned, ser::Serialize};

use super::Result;

pub trait Provider {
    fn get<K, V, F>(&mut self, key: &K, fun: F, ttl: Duration) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Display,
        V: DeserializeOwned + Serialize;
    fn clear(&mut self) -> Result<()>;
    fn keys(&mut self) -> Result<Vec<(String, i64)>>;
    fn version(&mut self) -> Result<String>;
    fn heartbeat(&mut self) -> Result<()>;
}

pub trait Kv {
    fn set<K: Display, V: Serialize>(&mut self, key: &K, val: &V) -> Result<()>;
    fn get<K: Display, V: DeserializeOwned>(&mut self, key: &K) -> Result<V>;
}
