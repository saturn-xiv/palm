pub mod redis;

use std::fmt::Display;

use chrono::Duration;
use serde::{de::DeserializeOwned, ser::Serialize};

use super::Result;

pub trait Provider {
    fn fetch<K, V>(&mut self, namespace: &str, key: &K) -> Result<V>
    where
        K: Display,
        V: DeserializeOwned;
    fn set<K, V>(&mut self, namespace: &str, key: &K, val: &V, ttl: Duration) -> Result<()>
    where
        K: Display,
        V: Serialize;
    fn get<K, V, F>(&mut self, namespace: &str, key: &K, fun: F, ttl: Duration) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Display,
        V: DeserializeOwned + Serialize;
    fn clear(&mut self, namespace: &str) -> Result<()>;
    fn keys(&mut self, namespace: &str) -> Result<Vec<(String, String, i64)>>;
    fn version(&mut self) -> Result<String>;
    fn heartbeat(&mut self) -> Result<()>;
    fn destroy<K: Display>(&mut self, namespace: &str, key: &K) -> Result<()>;

    fn key<K: Display>(namespace: &str, key: &K) -> String {
        format!("{}://{}", namespace, key)
    }
}
