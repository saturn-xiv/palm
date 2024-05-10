use std::fmt::{self};
use std::str::FromStr;

use data_encoding::BASE64URL_NOPAD;
use prost::Message as ProtobufMessage;

use super::super::{Error, Result};
use super::v1::UserOauth2State;

impl fmt::Display for UserOauth2State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let it = {
            let buf = self.encode_to_vec();
            BASE64URL_NOPAD.encode(&buf)
        };
        write!(f, "{}", it)
    }
}

impl FromStr for UserOauth2State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let buf = BASE64URL_NOPAD.decode(s.as_bytes())?;
        let it = ProtobufMessage::decode(&buf[..])?;
        Ok(it)
    }
}
