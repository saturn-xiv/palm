pub mod controllers;
pub mod graphql;
pub mod services;

use phlox::{Mac, Result, base64, random};
use prost::Message as ProtobufMessage;

use super::super::palm::portal::v1::HashedPassword;

pub const ISSUER: &str = "phlox.portal";

impl HashedPassword {
    pub fn new<P: AsRef<str>>(password: P) -> Self {
        let password = password.as_ref();
        let password = password.as_bytes();
        Self {
            data: password.to_vec(),
            salt: random::bytes(8),
        }
    }

    pub fn sign<H: Mac, P: AsRef<str>>(&self, mac: &H) -> Result<String> {
        let tmp = Self {
            data: mac.sign(&self.to_vec()?)?,
            salt: self.salt.clone(),
        };
        let it = base64::encode(&tmp.to_vec()?);
        Ok(it)
    }
    pub fn verify<H: Mac, S: AsRef<str>, P: AsRef<str>>(
        mac: &H,
        hash: S,
        password: P,
    ) -> Result<()> {
        let hash = hash.as_ref();
        let hash = hash.as_bytes();

        let buf = {
            let password = password.as_ref();
            let password = password.as_bytes();
            let mut it: Self = ProtobufMessage::decode(hash)?;
            it.data = password.to_vec();
            it.to_vec()?
        };
        mac.verify(hash, &buf)
    }

    fn to_vec(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        ProtobufMessage::encode(self, &mut buf)?;
        Ok(buf)
    }
}
