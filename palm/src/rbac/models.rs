use std::any::type_name;
use std::fmt;
use std::str::FromStr;

use bytes::Bytes;
use data_encoding::BASE64URL_NOPAD;
use prost::Message;

use super::super::{Error, Result};
use super::v1;

impl v1::Resource {
    pub fn by_i<T>(id: Option<i32>) -> Self {
        Self {
            r#type: type_name::<T>().to_string(),
            id: id.map(v1::resource::Id::I),
        }
    }
    pub fn by_s<T>(id: &str) -> Self {
        Self {
            r#type: type_name::<T>().to_string(),
            id: Some(v1::resource::Id::S(id.to_string())),
        }
    }
}

impl fmt::Display for v1::Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf = self.encode_to_vec();
        let buf = BASE64URL_NOPAD.encode(&buf);
        write!(f, "{}", buf)
    }
}

impl FromStr for v1::Resource {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let buf = BASE64URL_NOPAD.decode(s.as_bytes())?;
        let buf = Bytes::from(buf);
        let it = Self::decode(buf)?;
        Ok(it)
    }
}

impl fmt::Display for v1::User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf = self.encode_to_vec();
        let buf = BASE64URL_NOPAD.encode(&buf);
        write!(f, "{}", buf)
    }
}

impl FromStr for v1::User {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let buf = BASE64URL_NOPAD.decode(s.as_bytes())?;
        let buf = Bytes::from(buf);
        let it = Self::decode(buf)?;
        Ok(it)
    }
}
impl fmt::Display for v1::Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf = self.encode_to_vec();
        let buf = BASE64URL_NOPAD.encode(&buf);
        write!(f, "{}", buf)
    }
}

impl FromStr for v1::Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let buf = BASE64URL_NOPAD.decode(s.as_bytes())?;
        let buf = Bytes::from(buf);
        let it = Self::decode(buf)?;
        Ok(it)
    }
}
