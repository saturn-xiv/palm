use hyacinth::loquat_v1::{AesDecryptRequest, AesEncryptRequest};

use super::super::{HttpError, Loquat, Result, SecretBox, random::bytes as random_bytes};

impl SecretBox for Loquat {
    async fn encrypt(&self, plain: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let associated_data = random_bytes(32);
        let mut req = AesEncryptRequest::new();
        req.set_plain(plain);
        req.set_associated_data(&associated_data);

        let res = self
            .aes
            .encrypt(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok((res.cipher().to_vec(), associated_data))
    }
    async fn decrypt(&self, cipher: &[u8], associated_data: &[u8]) -> Result<Vec<u8>> {
        let mut req = AesDecryptRequest::new();
        req.set_cipher(cipher);
        req.set_associated_data(associated_data);

        let res = self
            .aes
            .decrypt(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(res.plain().to_vec())
    }
}
