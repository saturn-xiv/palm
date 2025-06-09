use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::super::Result;
use super::Jwt as JwtProvider;

#[derive(Clone)]
pub struct Jwt {
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub sub: String,
    pub aud: String,
    pub iss: String,
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
}

impl JwtProvider for Jwt {
    fn sign(&self, issuer: &str, subject: &str, audience: &str, ttl: Duration) -> Result<String> {
        let (nbf, exp) = Self::timestamps(ttl);
        let token = Token {
            sub: subject.to_string(),
            iss: issuer.to_string(),
            aud: audience.to_string(),
            iat: Utc::now().timestamp(),
            exp,
            nbf,
        };
        self.sum(None, &token)
    }
    fn verify(&self, token: &str, issuer: &str, audience: &str) -> Result<String> {
        let token: TokenData<Token> = self.parse(token, issuer, audience)?;
        Ok(token.claims.sub)
    }
}

impl Jwt {
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
        let mut vat = Validation::new(Algorithm::HS512);
        vat.leeway = 60;
        vat.set_audience(&[audience]);
        vat.set_issuer(&[issuer]);
        let val = decode(token, &DecodingKey::from_base64_secret(&self.key)?, &vat)?;
        Ok(val)
    }
}
