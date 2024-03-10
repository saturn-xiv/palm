use data_encoding::BASE64;
use openssl::{
    hash::MessageDigest,
    memcmp,
    pkey::{PKey, Private},
    sign::Signer,
};

use super::super::Result;

// https://docs.rs/openssl/latest/openssl/sign/index.html
pub struct Hmac {
    key: PKey<Private>,
    digest: MessageDigest,
}

impl Hmac {
    pub fn new(key: &str) -> Result<Self> {
        let key = BASE64.decode(key.as_bytes())?;
        Ok(Self {
            key: PKey::hmac(&key)?,
            digest: MessageDigest::sha512(),
        })
    }
}

impl super::Password for Hmac {
    fn compute(&self, plain_text: &[u8]) -> Result<Vec<u8>> {
        let mut signer = Signer::new(self.digest, &self.key)?;
        signer.update(plain_text)?;
        let cipher = signer.sign_to_vec()?;
        Ok(cipher)
    }
    fn verify(&self, cipher_text: &[u8], plain_text: &[u8]) -> bool {
        if let Ok(buf) = self.compute(plain_text) {
            return memcmp::eq(&buf, cipher_text);
        }
        false
    }
}
