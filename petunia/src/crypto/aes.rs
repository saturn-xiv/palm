use data_encoding::BASE64;
use openssl::symm::{Cipher, Crypter, Mode};

use super::super::Result;
use super::random::bytes as random_bytes;

// Serpent > Twofish > Serpent
pub struct OpenSsl {
    key: Vec<u8>, // 32 bytes
    cipher: Cipher,
}

impl OpenSsl {
    pub fn new(key: &str) -> Result<Self> {
        let key = BASE64.decode(key.as_bytes())?;
        Ok(Self {
            key,
            cipher: Cipher::aes_256_cbc(),
        })
    }
}

impl super::Secret for OpenSsl {
    fn encrypt(&self, plain: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let iv = random_bytes(self.cipher.block_size());
        let mut enc = Crypter::new(self.cipher, Mode::Encrypt, &self.key, Some(&iv))?;
        let mut cipher = vec![0; plain.len() + self.cipher.block_size()];
        let mut count = enc.update(plain, &mut cipher)?;
        count += enc.finalize(&mut cipher[count..])?;
        cipher.truncate(count);
        Ok((cipher, iv))
    }

    fn decrypt(&self, cipher: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let mut dec = Crypter::new(self.cipher, Mode::Decrypt, &self.key, Some(iv))?;
        let mut plain = vec![0; cipher.len() + self.cipher.block_size()];

        let mut count = dec.update(cipher, &mut plain)?;
        count += dec.finalize(&mut plain[count..])?;
        plain.truncate(count);

        Ok(plain)
    }
}
