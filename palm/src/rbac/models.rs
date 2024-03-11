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

impl v1::Role {
    pub fn administrator() -> Self {
        Self {
            by: Some(v1::role::By::Administrator(())),
        }
    }
    pub fn root() -> Self {
        Self {
            by: Some(v1::role::By::Root(())),
        }
    }
    pub fn member(member: String) -> Self {
        Self {
            by: Some(v1::role::By::Member(member)),
        }
    }
}

impl v1::Permission {
    pub fn all(rules: &[Vec<String>]) -> Vec<Self> {
        let mut items = Vec::new();
        for rule in rules.iter() {
            if rule.len() == 4 && rule[0] == "p" {
                items.push(Self {
                    resource: rule[2].parse::<v1::Resource>().ok(),
                    action: rule[3].clone(),
                });
            }
        }
        items
    }
}
