pub mod aead;
pub mod base64;
pub mod cache;
pub mod hmac;
pub mod jwt;
pub mod minio;
pub mod open_search;
pub mod orm;
pub mod queue;
pub mod random;
pub mod ssha512;

use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;
use std::str::FromStr;

use data_encoding::{BASE64_NOPAD, DecodeError as Base64DecodeError};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub type Error = Box<dyn StdError + Send + Sync>;
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub struct HttpError(pub StatusCode, pub Option<String>);
impl StdError for HttpError {}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            Some(ref v) => v.fmt(f),
            None => self.0.fmt(f),
        }
    }
}

impl From<Error> for HttpError {
    fn from(err: Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, Some(err.to_string()))
    }
}

pub type HttpResult<T> = StdResult<T, HttpError>;
pub type GrpcResult<T> = StdResult<tonic::Response<T>, tonic::Status>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key(pub Vec<u8>);

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", BASE64_NOPAD.encode(&self.0))
    }
}
impl FromStr for Key {
    type Err = Base64DecodeError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut buf = vec![0; BASE64_NOPAD.decode_len(s.len())?];
        BASE64_NOPAD
            .decode_mut(s.as_bytes(), &mut buf)
            .map_err(|x| x.error)?;
        Ok(Self(buf))
    }
}

pub trait Mac {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, hash: &[u8], message: &[u8]) -> Result<()>;
}
pub trait Enigma {
    fn encrypt(&self, plain: &[u8]) -> Result<(Vec<u8>, Vec<u8>)>;
    fn decrypt(&self, code: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;
}
