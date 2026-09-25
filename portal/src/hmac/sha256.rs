use openssl::{
    error::ErrorStack,
    hash::MessageDigest,
    memcmp,
    pkey::{PKey, Private},
    sign::Signer,
};

pub struct HmacSha256 {
    key: PKey<Private>,
}
impl HmacSha256 {
    pub fn new(key: &[u8]) -> Result<Self, ErrorStack> {
        let it = Self {
            key: PKey::hmac(key)?,
        };
        Ok(it)
    }
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.key)?;
        signer.update(data)?;
        let it = signer.sign_to_vec()?;
        Ok(it)
    }
    pub fn verify(&self, hash: &[u8], data: &[u8]) -> Result<bool, ErrorStack> {
        let tmp = self.sign(data)?;
        Ok(memcmp::eq(&tmp, hash))
    }
}
