pub mod redis;

use std::time::Duration;

use hyacinth::ProtobufMessage;
use serde::{Serialize, de::DeserializeOwned};

use super::Result;

pub trait ProtobufCacher {
    fn set<K: AsRef<str>, V: ProtobufMessage>(
        &self,
        key: K,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()>;
    fn get<K: AsRef<str>, V: ProtobufMessage + Default>(&self, key: K) -> Result<V>;
}

pub trait FlexBuffersCacher {
    fn set<K: AsRef<str>, V: Serialize>(
        &self,
        key: K,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()>;
    fn get<K: AsRef<str>, V: DeserializeOwned>(&self, key: K) -> Result<V>;
}
