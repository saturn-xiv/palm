pub mod aes;
pub mod hmac;
pub mod random;
pub mod sha1;
pub mod ssha512;

use data_encoding::BASE64;
use serde::{Deserialize, Serialize};

use super::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key(pub String);

impl Default for Key {
    fn default() -> Self {
        Self(BASE64.encode(&random::bytes(32)))
    }
}

impl From<Key> for Result<Vec<u8>> {
    fn from(it: Key) -> Self {
        let buf = BASE64.decode(it.0.as_bytes())?;
        Ok(buf)
    }
}
