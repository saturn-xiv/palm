use chrono::NaiveDateTime;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::super::Result;

#[derive(Clone)]
pub struct Jwt {
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub jti: String,
    pub ext: Option<Vec<u8>>,
    pub nbf: i64,
    pub exp: i64,
    pub iat: i64,
}

impl super::super::Jwt for Jwt {
    fn verify<T: DeserializeOwned>(
        &self,
        token: &str,
        issuer: &str,
        audience: &str,
    ) -> Result<(String, String, Option<T>)> {
        let token: TokenData<Token> = self.parse(token, issuer, audience)?;
        let payload = if let Some(ref ext) = token.claims.ext {
            let it = flexbuffers::from_slice(ext)?;
            Some(it)
        } else {
            None
        };

        Ok((token.claims.jti, token.claims.sub, payload))
    }
    fn sign<S: Serialize>(
        &self,
        jwt_id: &str,
        issuer: &str,
        subject: &str,
        audience: &str,
        payload: &Option<S>,
        (issued_at, not_before, expired_at): (NaiveDateTime, NaiveDateTime, NaiveDateTime),
    ) -> Result<String> {
        let ext = if let Some(payload) = payload {
            let it = flexbuffers::to_vec(payload)?;
            Some(it)
        } else {
            None
        };
        let token = Token {
            iss: issuer.to_string(),
            sub: subject.to_string(),
            aud: audience.to_string(),
            ext,
            jti: jwt_id.to_string(),
            iat: issued_at.and_utc().timestamp(),
            exp: expired_at.and_utc().timestamp(),
            nbf: not_before.and_utc().timestamp(),
        };
        self.sum(None, &token)
    }
}

impl Jwt {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
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
