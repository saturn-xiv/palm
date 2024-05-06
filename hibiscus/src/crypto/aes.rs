use data_encoding::BASE64;
use openssl::symm::{decrypt, encrypt, Cipher};

use super::super::Result;
use super::random::bytes as random_bytes;

// https://docs.rs/openssl/latest/openssl/symm/index.html
// Serpent > Twofish > Serpent
pub struct Aes {
    key: Vec<u8>, // 32 bytes
    cipher: Cipher,
}

impl Aes {
    pub fn new(key: &str) -> Result<Self> {
        let key = BASE64.decode(key.as_bytes())?;
        Ok(Self {
            key,
            cipher: Cipher::aes_256_gcm(),
        })
    }
}

impl super::Secret for Aes {
    fn encrypt(&self, plain_text: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let iv = random_bytes(self.cipher.block_size());
        let cipher_text = encrypt(self.cipher, &self.key, Some(&iv), plain_text)?;
        Ok((cipher_text, iv))
    }

    fn decrypt(&self, cipher_text: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let plain_text: Vec<u8> = decrypt(self.cipher, &self.key, Some(iv), cipher_text)?;
        Ok(plain_text)
    }
}
