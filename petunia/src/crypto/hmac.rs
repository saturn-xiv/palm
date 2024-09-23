use data_encoding::BASE64;
use openssl::{
    hash::MessageDigest,
    memcmp,
    pkey::{PKey, Private},
    sign::Signer,
};

use super::super::Result;

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
    fn sign(&self, plain: &[u8]) -> Result<Vec<u8>> {
        let mut signer = Signer::new(self.digest, &self.key)?;
        signer.update(plain)?;
        let cipher = signer.sign_to_vec()?;
        Ok(cipher)
    }
    fn verify(&self, cipher: &[u8], plain: &[u8]) -> bool {
        if let Ok(buf) = self.sign(plain) {
            return memcmp::eq(&buf, cipher);
        }
        false
    }
}
