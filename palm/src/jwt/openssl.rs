use chrono::Duration;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use super::super::Result;
use super::Jwt;

#[derive(Clone)]
pub struct OpenSsl {
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub jti: String,
    pub nbf: i64,
    pub exp: i64,
    pub iat: i64,
}

impl Jwt for OpenSsl {
    fn sign(&self, issuer: &str, subject: &str, audience: &str, ttl: Duration) -> Result<String> {
        let (iat, nbf, exp) = Self::timestamps(ttl);
        let token = Token {
            iss: issuer.to_string(),
            sub: subject.to_string(),
            aud: audience.to_string(),
            jti: Uuid::new_v4().to_string(),
            iat,
            exp,
            nbf,
        };
        self.sum(None, &token)
    }
    fn verify(&self, token: &str, issuer: &str, audience: &str) -> Result<(String, String)> {
        let token: TokenData<Token> = self.parse(token, issuer, audience)?;
        Ok((token.claims.jti, token.claims.sub))
    }
}

impl OpenSsl {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    fn sum<T: Serialize>(&self, kid: Option<String>, claims: &T) -> Result<String> {
        let token = encode(
            &Header {
                kid,
                alg: Algorithm::HS512,
                ..Default::default()
            },
            claims,
            &EncodingKey::from_base64_secret(&self.key)?,
        )?;
        Ok(token)
    }
    fn parse<T: DeserializeOwned>(
        &self,
        token: &str,
        issuer: &str,
        audience: &str,
    ) -> Result<TokenData<T>> {
        let vat = {
            let mut it = Validation::new(Algorithm::HS512);
            it.leeway = 60;
            it.set_issuer(&[issuer]);
            it.set_audience(&[audience]);
            it
        };
        let val = decode(token, &DecodingKey::from_base64_secret(&self.key)?, &vat)?;
        Ok(val)
    }
}
