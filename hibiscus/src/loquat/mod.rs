pub mod protocols;

use std::collections::{BTreeMap, BTreeSet};

use chrono::NaiveDateTime;
use hyper::StatusCode;

use super::{
    crypto::random::bytes as random_bytes,
    crypto::{Password, Secret},
    env::Thrift,
    jwt::Jwt,
    HttpError, Result,
};

use self::protocols::{
    AesSyncClient, HealthSyncClient, HmacSyncClient, JwtSignRequest, JwtSyncClient, TAesSyncClient,
    THealthSyncClient, THmacSyncClient, TJwtSyncClient,
};

const AES: &str = "N6loquat2v15AesIfE";
const JWT: &str = "N6loquat2v15JwtIfE";
const HMAC: &str = "N6loquat2v16HmacIfE";
const HEALTH: &str = "N6loquat2v18HealthIfE";

impl Secret for Thrift {
    fn encrypt(&self, plain_text: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let (i_prot, o_prot) = self.open(AES)?;
        let mut client: AesSyncClient<_, _> = AesSyncClient::new(i_prot, o_prot);
        let salt = random_bytes(32);
        let mut buf = Vec::new();
        buf.extend_from_slice(&salt);
        buf.extend_from_slice(plain_text);
        let cipher = client.encrypt(buf)?;
        Ok((cipher, salt))
    }
    fn decrypt(&self, cipher_text: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let (i_prot, o_prot) = self.open(AES)?;
        let mut client: AesSyncClient<_, _> = AesSyncClient::new(i_prot, o_prot);
        let buf = client.decrypt(cipher_text.to_vec())?;

        let ix = iv.len();
        let salt = &buf[..ix];
        let plain = &buf[ix..];
        if salt != iv {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("bad salt".to_string()),
            )));
        }
        Ok(plain.to_vec())
    }
}

impl Password for Thrift {
    fn compute(&self, plain_text: &[u8], salt_len: usize) -> Result<(Vec<u8>, Vec<u8>)> {
        let (i_prot, o_prot) = self.open(HMAC)?;
        let mut client: HmacSyncClient<_, _> = HmacSyncClient::new(i_prot, o_prot);
        let salt = random_bytes(salt_len);
        let mut buf = Vec::new();
        buf.extend_from_slice(plain_text);
        buf.extend_from_slice(&salt);
        let cipher = client.sign(buf)?;
        Ok((cipher, salt))
    }
    fn verify(&self, cipher_text: &[u8], plain_text: &[u8], salt: &[u8]) -> bool {
        if let Ok((i_prot, o_prot)) = self.open(HMAC) {
            let mut client: HmacSyncClient<_, _> = HmacSyncClient::new(i_prot, o_prot);
            let mut buf = Vec::new();
            buf.extend_from_slice(plain_text);
            buf.extend_from_slice(salt);
            return client.verify(cipher_text.to_vec(), buf).is_ok();
        }
        false
    }
}

pub trait HealthCheck {
    fn check(&self) -> Result<BTreeMap<String, String>>;
}

impl HealthCheck for Thrift {
    fn check(&self) -> Result<BTreeMap<String, String>> {
        let (i_prot, o_prot) = self.open(HEALTH)?;
        let mut client: HealthSyncClient<_, _> = HealthSyncClient::new(i_prot, o_prot);
        let res = client.check()?;
        Ok(res)
    }
}

impl Jwt for Thrift {
    fn verify(&self, token: &str, issuer: &str, audience: &str) -> Result<(String, String)> {
        let (i_prot, o_prot) = self.open(JWT)?;
        let mut client = JwtSyncClient::new(i_prot, o_prot);
        let res = client.verify(token.to_string(), issuer.to_string(), audience.to_string())?;
        let jwt_id = res.jwt_id.as_ref().ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("empty jwt-id header".to_string()),
        )))?;
        Ok((jwt_id.clone(), res.subject))
    }
    fn sign(
        &self,
        jwt_id: &str,
        issuer: &str,
        subject: &str,
        audience: &str,
        (issued_at, not_before, expired_at): (NaiveDateTime, NaiveDateTime, NaiveDateTime),
    ) -> Result<String> {
        let (i_prot, o_prot) = self.open(JWT)?;
        let mut client = JwtSyncClient::new(i_prot, o_prot);
        let req = JwtSignRequest {
            jwt_id: Some(jwt_id.to_string()),
            key_id: None,
            issuer: issuer.to_string(),
            subject: subject.to_string(),
            audiences: {
                let mut items = BTreeSet::new();
                items.insert(audience.to_string());
                items
            },
            issued_at: issued_at.and_utc().timestamp(),
            expired_at: expired_at.and_utc().timestamp(),
            not_before: not_before.and_utc().timestamp(),
            payload: None,
        };
        let token = client.sign(req)?;
        Ok(token)
    }
}
