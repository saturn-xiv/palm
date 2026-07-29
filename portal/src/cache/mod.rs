pub mod redis;

use std::time::Duration;

use hyacinth::ProtobufMessage;
use serde::{Serialize, de::DeserializeOwned};

use super::Result;

pub trait ProtobufCacher {
    fn set<K: AsRef<str>, V: ProtobufMessage>(
        &mut self,
        key: K,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()>;
    fn get<K: AsRef<str>, V: ProtobufMessage + Default>(&mut self, key: K) -> Result<V>;
}

pub trait FlexBuffersCacher {
    fn set<K: AsRef<str>, V: Serialize>(
        &mut self,
        key: K,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()>;
    fn get<K: AsRef<str>, V: DeserializeOwned>(&mut self, key: K) -> Result<V>;
}
