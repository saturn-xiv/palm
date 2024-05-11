pub mod azalea;
pub mod camelia;
pub mod daisy;
pub mod gourd;
pub mod iso4217;
pub mod jasmine;
pub mod lily;
pub mod loquat;
pub mod morus;
pub mod musa;
pub mod openssl;
pub mod orchid;
pub mod random;
pub mod tuberose;

use std::error::Error as StdError;
use std::fmt;
use std::ops::Add;
use std::result::Result as StdResult;

use chrono::{Datelike, Duration, NaiveDateTime, Utc};
use data_encoding::BASE64;
use hyper::{header::AUTHORIZATION, StatusCode};
use serde::{Deserialize, Serialize};
use thrift::{
    protocol::{
        TBinaryInputProtocol, TBinaryOutputProtocol, TMultiplexedOutputProtocol, TOutputProtocol,
        TSerializable,
    },
    transport::{
        ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TFramedReadTransport,
        TFramedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
    },
    Result as ThriftResult,
};
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataValue},
    Request as GrpcRequest,
};

#[macro_export]
macro_rules! to_timestamp {
    ($x:expr) => {{
        let it: std::time::SystemTime =
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset($x, chrono::Utc).into();
        it.into()
    }};
}

#[macro_export]
macro_rules! to_datetime {
    ($x:expr) => {{
        chrono::NaiveDateTime::from_timestamp_opt($x.seconds, $x.nanos as u32).unwrap_or_default()
    }};
}

#[macro_export]
macro_rules! to_chrono_duration {
    ($x:expr) => {{
        chrono::Duration::seconds($x.seconds)
    }};
}

#[macro_export]
macro_rules! to_utc_datetime {
    ($x:expr) => {{
        chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset($x, chrono::Utc)
    }};
}

#[macro_export]
macro_rules! to_std_duration {
    ($x:expr) => {{
        std::time::Duration::new($x.seconds as u64, $x.nanos as u32)
    }};
}

#[macro_export]
macro_rules! to_code {
    ($x:expr) => {{
        let it = $x.trim();
        it.to_lowercase()
    }};
}

#[macro_export]
macro_rules! try_thrift {
    ($r:expr, $e:expr) => {{
        $r.map_err($e)
    }};
    ($r:expr) => {{
        try_thrift!($r, |e| Error::Application(ApplicationError::new(
            ApplicationErrorKind::InternalError,
            e.to_string(),
        )))
    }};
}

#[macro_export]
macro_rules! try_graphql {
    ($r:expr, $e:expr) => {{
        $r.map_err($e)
    }};
    ($r:expr) => {{
        try_grpc!($r, |e| tonic::Status::internal(e.to_string()))
    }};
}

#[macro_export]
macro_rules! try_grpc {
    ($r:expr, $e:expr) => {{
        $r.map_err($e)
    }};
    ($r:expr) => {{
        try_grpc!($r, |e| tonic::Status::internal(e.to_string()))
    }};
}

#[macro_export]
macro_rules! try_web {
    ($r:expr, $e:expr) => {{
        $r.map_err($e)
    }};
    ($r:expr) => {{
        try_web!($r, actix_web::error::ErrorInternalServerError)
    }};
}

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &str = include_str!("banner.txt");

pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

include!(concat!(env!("OUT_DIR"), "/env.rs"));

lazy_static::lazy_static! {
    pub static ref VERSION: String = format!("{GIT_VERSION}({BUILD_TIME})");
}

// https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-WEB.md
// https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md
// https://developers.cloudflare.com/support/speed/optimization-file-size/what-will-cloudflare-compress/
pub const PROTOBUF: &str = "application/x-protobuf";
pub const FLATBUFFER: &str = "application/x-flatbuffer";

pub type Error = Box<dyn StdError + Send + Sync>;
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub struct HttpError(pub StatusCode, pub Option<String>);
pub type HttpResult<T> = StdResult<T, HttpError>;
pub type GrpcResult<T> = StdResult<tonic::Response<T>, tonic::Status>;

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

pub trait Password {
    fn compute(&self, plain_text: &[u8], salt_len: usize) -> Result<(Vec<u8>, Vec<u8>)>;
    fn verify(&self, cipher_text: &[u8], plain_text: &[u8], salt: &[u8]) -> bool;
}

pub trait Secret {
    fn encrypt(&self, plain_text: &[u8]) -> Result<(Vec<u8>, Vec<u8>)>;
    fn decrypt(&self, cipher_text: &[u8], iv: &[u8]) -> Result<Vec<u8>>;
}

pub const BEARER: &str = "Bearer ";

// https://www.ibm.com/support/knowledgecenter/zh/SSEQTP_8.5.5/com.ibm.websphere.wlp.doc/ae/cwlp_jwttoken.html
// https://jwt.io/
// https://tools.ietf.org/html/rfc7519
pub trait Jwt {
    fn verify(&self, token: &str, issuer: &str, audience: &str) -> Result<(String, String)>;
    fn sign(
        &self,
        jwt_id: &str,
        issuer: &str,
        subject: &str,
        audience: &str,
        timestamps: (NaiveDateTime, NaiveDateTime, NaiveDateTime),
    ) -> Result<String>;

    fn sign_by_duration(
        &self,
        issuer: &str,
        subject: &str,
        audience: &str,
        ttl: Duration,
    ) -> Result<String> {
        let (iat, nbf, exp) = Self::timestamps(ttl)?;
        self.sign(&random::uuid(), issuer, subject, audience, (iat, nbf, exp))
    }
    fn sign_by_range(
        &self,
        issuer: &str,
        subject: &str,
        audience: &str,
        not_before: NaiveDateTime,
        expired_at: NaiveDateTime,
    ) -> Result<String> {
        self.sign(
            &random::uuid(),
            issuer,
            subject,
            audience,
            (Utc::now().naive_local(), not_before, expired_at),
        )
    }

    fn bearer(token: &str) -> String {
        format!("{}{}", BEARER, token)
    }
    fn authorization<R>(request: &mut GrpcRequest<R>, token: &str) -> Result<()> {
        let val: MetadataValue<Ascii> = {
            let it = Self::bearer(token);
            it.parse()?
        };

        let key: MetadataKey<Ascii> = {
            let it = AUTHORIZATION.as_str();
            let it = it.to_lowercase();
            it.parse()?
        };
        request.metadata_mut().insert(key, val);

        log::debug!("request header {:?}", request.metadata());
        Ok(())
    }

    fn timestamps(ttl: Duration) -> Result<(NaiveDateTime, NaiveDateTime, NaiveDateTime)> {
        let now = Utc::now();
        let nbf = now.add(Duration::try_seconds(-1).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad seconds".to_string()),
        )))?);
        let exp = now.add(ttl);
        Ok((now.naive_local(), nbf.naive_local(), exp.naive_local()))
    }
    fn years(y: i32) -> Result<(i64, i64, i64)> {
        let now = Utc::now();
        let nbf = now.add(Duration::try_seconds(-1).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad seconds".to_string()),
        )))?);
        let exp = now.with_year(nbf.year() + y).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad years".to_string()),
        )))?;
        Ok((now.timestamp(), nbf.timestamp(), exp.timestamp()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key(pub String);

impl Default for Key {
    fn default() -> Self {
        Self(BASE64.encode(&random::bytes(32)))
    }
}

impl From<Key> for Result<Vec<u8>> {
    fn from(it: Key) -> Self {
        let buf = BASE64.decode(it.0.as_bytes())?;
        Ok(buf)
    }
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
}

pub fn to_bytes<T: TSerializable>(val: &T) -> ThriftResult<Vec<u8>> {
    let mut buf = Vec::new();
    {
        let mut output = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(&mut buf), true);
        val.write_to_out_protocol(&mut output)?;
        output.flush()?;
    }

    Ok(buf)
}

pub fn from_bytes<T: TSerializable>(buf: &[u8]) -> ThriftResult<T> {
    let mut input = Box::new(TBinaryInputProtocol::new(
        TBufferedReadTransport::with_capacity(1 << 12, buf),
        true,
    ));
    let val = T::read_from_in_protocol(&mut input)?;
    Ok(val)
}
