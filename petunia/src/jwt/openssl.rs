use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::super::Result;
use super::Jwt;

#[derive(Clone)]
pub struct OpenSsl {
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub sub: String,
    pub aud: String,
    pub nbf: i64,
    pub exp: i64,
}

impl Jwt for OpenSsl {
    fn sign(
        &self,
        subject: &str,
        audience: &str,
        not_before: i64,
        expires_at: i64,
    ) -> Result<String> {
        let token = Token {
            sub: subject.to_string(),
            aud: audience.to_string(),
            exp: expires_at,
            nbf: not_before,
        };
        self.sum(None, &token)
    }
    fn verify(&self, token: &str, audience: &str) -> Result<String> {
        let token: TokenData<Token> = self.parse(token, audience)?;
        Ok(token.claims.sub)
    }
}

impl OpenSsl {
    // openssl rand -base64 128
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn sum<T: Serialize>(&self, kid: Option<String>, claims: &T) -> Result<String> {
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
    pub fn parse<T: DeserializeOwned>(&self, token: &str, audience: &str) -> Result<TokenData<T>> {
        let mut vat = Validation::new(Algorithm::HS512);
        vat.leeway = 60;
        vat.set_audience(&[audience]);
        let val = decode(token, &DecodingKey::from_base64_secret(&self.key)?, &vat)?;
        Ok(val)
    }
}
