use std::time::Duration as StdDuration;

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use super::{Result, random::uuid};

// https://www.jwt.io/
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    aud: String,
    exp: usize,
    nbf: usize,
    iat: usize,
}

pub struct JwtHS512 {
    key: Vec<u8>,
}

impl JwtHS512 {
    pub fn new(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }
}

impl super::Jwt for JwtHS512 {
    fn sign(
        &self,
        issuer: &str,
        subject: &str,
        audience: &str,
        ttl: StdDuration,
    ) -> Result<String> {
        let now = Utc::now();
        let mut header = Header::new(Algorithm::HS512);
        header.kid = Some(uuid());
        let it = encode(
            &header,
            &Claims {
                iss: issuer.to_string(),
                aud: audience.to_string(),
                sub: subject.to_string(),
                iat: now.timestamp() as usize,
                nbf: (now + Duration::seconds(1)).timestamp() as usize,
                exp: (now + ttl).timestamp() as usize,
            },
            &EncodingKey::from_secret(&self.key),
        )?;
        Ok(it)
    }
    fn verify<T: ToString>(&self, token: &str, issuer: &str, audiences: &[T]) -> Result<String> {
        let it = decode::<Claims>(&token, &DecodingKey::from_secret(&self.key), &{
            let mut it = Validation::new(Algorithm::HS512);
            it.set_issuer(&[issuer]);
            it.set_audience(audiences);
            it
        })?;
        Ok(it.claims.sub)
    }
}
