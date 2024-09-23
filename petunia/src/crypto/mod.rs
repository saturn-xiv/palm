pub mod aes;
pub mod hmac;
pub mod random;
pub mod sha1;
pub mod ssha512;

use data_encoding::BASE64;
use serde::{Deserialize, Serialize};

use super::Result;

pub trait Password {
    fn sign(&self, plain: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, cipher: &[u8], plain: &[u8]) -> bool;
}

pub trait Secret {
    fn encrypt(&self, plain: &[u8]) -> Result<(Vec<u8>, Vec<u8>)>;
    fn decrypt(&self, cipher: &[u8], iv: &[u8]) -> Result<Vec<u8>>;
}

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
