pub mod v1;

use std::collections::BTreeSet;

use chrono::{Duration, Utc};

use super::{
    Result,
    crypto::{Password, Secret},
    jwt::Jwt,
    thrift::Thrift,
};

use v1::{
    AesSyncClient, HMacSyncClient, HealthSyncClient, JwtSignRequest, JwtSyncClient, TAesSyncClient,
    THMacSyncClient, THealthSyncClient, TJwtSyncClient,
};

const JWT: &str = "N6loquat2v15JwtIfE";
const AES: &str = "N6loquat2v15AesIfE";
const HMAC: &str = "N6loquat2v16HMacIfE";
const HEALTH: &str = "N6loquat2v18HealthIfE";

impl Jwt for Thrift {
    fn sign(&self, issuer: &str, subject: &str, audience: &str, ttl: Duration) -> Result<String> {
        let now = Utc::now();
        let (not_before, expires_at) = Self::timestamps(ttl);
        let mut audiences = BTreeSet::new();
        audiences.insert(audience.to_string());
        let request = JwtSignRequest {
            jwt_id: None,
            key_id: None,
            payload: None,
            subject: subject.to_string(),
            issuer: issuer.to_string(),
            issued_at: now.timestamp(),
            audiences,
            not_before,
            expires_at,
        };
        let (i_prot, o_prot) = self.open(JWT)?;
        let mut client = JwtSyncClient::new(i_prot, o_prot);
        let token = client.sign(request)?;
        Ok(token)
    }

    fn verify(&self, token: &str, issuer: &str, audience: &str) -> Result<String> {
        let (i_prot, o_prot) = self.open(JWT)?;
        let mut client = JwtSyncClient::new(i_prot, o_prot);
        let response =
            client.verify(token.to_string(), issuer.to_string(), audience.to_string())?;
        Ok(response.subject)
    }
}

impl Password for Thrift {
    fn sign(&self, plain: &[u8]) -> Result<Vec<u8>> {
        let (i_prot, o_prot) = self.open(HMAC)?;
        let mut client = HMacSyncClient::new(i_prot, o_prot);
        let token = client.sign(plain.to_vec())?;
        Ok(token)
    }
    fn verify(&self, code: &[u8], plain: &[u8]) -> bool {
        if let Ok((i_prot, o_prot)) = self.open(HMAC) {
            let mut client = HMacSyncClient::new(i_prot, o_prot);
            return client.verify(code.to_vec(), plain.to_vec()).is_ok();
        }
        false
    }
}

impl Secret for Thrift {
    fn encrypt(&self, plain: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let (i_prot, o_prot) = self.open(AES)?;
        let mut client = AesSyncClient::new(i_prot, o_prot);
        let token = client.encrypt(plain.to_vec())?;
        Ok((token, Vec::new()))
    }
    fn decrypt(&self, code: &[u8], _iv: &[u8]) -> Result<Vec<u8>> {
        let (i_prot, o_prot) = self.open(AES)?;
        let mut client = AesSyncClient::new(i_prot, o_prot);
        let subject = client.decrypt(code.to_vec())?;
        Ok(subject)
    }
}

pub trait Health {
    fn check(&self) -> Result<()>;
}

impl Health for Thrift {
    fn check(&self) -> Result<()> {
        let (i_prot, o_prot) = self.open(HEALTH)?;
        let mut client = HealthSyncClient::new(i_prot, o_prot);
        client.check()?;
        Ok(())
    }
}
