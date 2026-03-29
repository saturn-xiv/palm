use aes_gcm_siv::{
    Aes256GcmSiv as Aes256GcmSiv_, Nonce,
    aead::{Aead, Key, KeyInit},
};
use crypto_common::Generate;

use super::Result;

pub struct Aes256GcmSiv {
    cipher: Aes256GcmSiv_,
}

impl Aes256GcmSiv {
    pub fn new(key: &[u8]) -> Result<Self> {
        let key = Key::<Aes256GcmSiv_>::try_from(key)?;
        Ok(Self {
            cipher: Aes256GcmSiv_::new(&key),
        })
    }
}

impl super::Enigma for Aes256GcmSiv {
    fn encrypt(&self, plain: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let nonce = Nonce::generate();
        let it = self.cipher.encrypt(&nonce, plain)?;
        Ok((nonce.to_vec(), it))
    }
    fn decrypt(&self, code: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::try_from(nonce)?;
        let it = self.cipher.decrypt(&nonce, code)?;
        Ok(it)
    }
}
