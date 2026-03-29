use hmac::{Hmac as Hmac_, KeyInit, Mac as Mac_};
use sha2::Sha512;

use super::Result;

type HmacSha512_ = Hmac_<Sha512>;

pub struct HmacSha512 {
    key: Vec<u8>,
}

impl HmacSha512 {
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }
}

impl super::Mac for HmacSha512 {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        let mut mac = HmacSha512_::new_from_slice(&self.key)?;
        mac.update(message);
        let it = mac.finalize();
        let it = it.into_bytes().to_vec();
        Ok(it)
    }
    fn verify(&self, hash: &[u8], message: &[u8]) -> Result<()> {
        let mut mac = HmacSha512_::new_from_slice(&self.key)?;
        mac.update(message);
        mac.verify_slice(hash)?;
        Ok(())
    }
}
