use data_encoding::BASE64;
use openssl::{
    hash::MessageDigest,
    memcmp,
    pkey::{PKey, Private},
    sign::Signer,
};

use super::super::Result;
use super::random::bytes as random_bytes;

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
    fn compute(&self, plain_text: &[u8], salt_len: usize) -> Result<(Vec<u8>, Vec<u8>)> {
        let salt = random_bytes(salt_len);
        let cipher = self.compute_(plain_text, &salt)?;
        Ok((cipher, salt))
    }
    fn verify(&self, cipher_text: &[u8], plain_text: &[u8], salt: &[u8]) -> bool {
        if let Ok(buf) = self.compute_(plain_text, salt) {
            return memcmp::eq(&buf, cipher_text);
        }
        false
    }
}

impl Hmac {
    fn compute_(&self, plain_text: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
        let mut signer = Signer::new(self.digest, &self.key)?;
        signer.update(plain_text)?;
        signer.update(salt)?;
        let cipher = signer.sign_to_vec()?;
        Ok(cipher)
    }
}
