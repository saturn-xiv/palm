use std::default::Default;
use std::fmt;

use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use thrift::{
    protocol::{
        TBinaryInputProtocol, TBinaryOutputProtocol, TMultiplexedOutputProtocol, TSerializable,
    },
    transport::{
        ReadHalf, TFramedReadTransport, TFramedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
    },
    Result as ThriftResult,
};

#[derive(Serialize, Deserialize, EnumDisplay, EnumString, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Environment {
    Production,
    Staging,
    #[default]
    Development,
    Test,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Thrift {
    pub host: String,
    pub port: u16,
    pub secure: bool,
}

impl Default for Thrift {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 9999,
            secure: false,
        }
    }
}
impl fmt::Display for Thrift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}://{}:{}",
            if self.secure { "tcps" } else { "tcp" },
            self.host,
            self.port
        )
    }
}

pub type ThriftInputProtocol = TBinaryInputProtocol<TFramedReadTransport<ReadHalf<TTcpChannel>>>;
pub type ThriftOutputProtocol = TMultiplexedOutputProtocol<
    TBinaryOutputProtocol<TFramedWriteTransport<WriteHalf<TTcpChannel>>>,
>;

impl Thrift {
    pub fn open(&self, service: &str) -> ThriftResult<(ThriftInputProtocol, ThriftOutputProtocol)> {
        let mut ch = TTcpChannel::new();
        ch.open(format!("{}:{}", self.host, self.port))?;

        let (i_chan, o_chan) = ch.split()?;

        let i_prot = TBinaryInputProtocol::new(TFramedReadTransport::new(i_chan), true);
        let o_prot = TMultiplexedOutputProtocol::new(
            service,
            TBinaryOutputProtocol::new(TFramedWriteTransport::new(o_chan), true),
        );
        Ok((i_prot, o_prot))
    }

    pub fn to_bytes<T: TSerializable>(val: &T, buf: &mut [u8]) -> ThriftResult<()> {
        let mut output = Box::new(TBinaryOutputProtocol::new(buf, true));
        val.write_to_out_protocol(&mut output)?;
        Ok(())
    }

    pub fn from_bytes<T: TSerializable>(buf: &[u8]) -> ThriftResult<T> {
        let mut input = Box::new(TBinaryInputProtocol::new(buf, true));
        let val = T::read_from_in_protocol(&mut input)?;
        Ok(val)
    }
}
