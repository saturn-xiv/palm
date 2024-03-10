pub mod aes;
pub mod hmac;
pub mod random;
pub mod sha1;
pub mod ssha512;
pub mod tink;

use data_encoding::BASE64;
use serde::{Deserialize, Serialize};

use super::Result;

pub trait Password {
    fn compute(&self, plain_text: &[u8], salt_len: usize) -> Result<(Vec<u8>, Vec<u8>)>;
    fn verify(&self, cipher_text: &[u8], plain_text: &[u8], salt: &[u8]) -> bool;
}

pub trait Secret {
    fn encrypt(&self, plain_text: &[u8]) -> Result<(Vec<u8>, Vec<u8>)>;
    fn decrypt(&self, cipher_text: &[u8], iv: &[u8]) -> Result<Vec<u8>>;
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
