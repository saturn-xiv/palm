pub mod aes;
pub mod argon2;
pub mod cache;
pub mod content_types;
pub mod controllers;
pub mod graphql;
pub mod gravatar;
pub mod headers;
pub mod hmac;
pub mod iso4217;
pub mod jwt;
pub mod mailer;
pub mod minio;
pub mod models;
pub mod open_search;
pub mod orm;
pub mod queue;
pub mod random;
pub mod rbac;
pub mod session;
pub mod ssha512;
pub mod twilio;

use std::env::current_exe;
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::result::Result as StdResult;
use std::str::FromStr;

use axum::response::Html;
use chrono::Duration;
use data_encoding::{BASE64, DecodeError as Base64DecodeError};
use hyacinth::{GrpcClientChannel, GrpcStatusError, loquat_v1, rbac_v1, wechat_pay_v1};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

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

impl From<GrpcStatusError> for HttpError {
    fn from(err: GrpcStatusError) -> Self {
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(err.message().to_string()),
        )
    }
}

pub type HttpResult<T> = StdResult<T, HttpError>;

#[macro_export]
macro_rules! web_try {
    ($x:expr) => {
        $x.map_err(|x| {
            log::error!("{}", x);
            (StatusCode::INTERNAL_SERVER_ERROR, x.to_string())
        })?
    };
}

pub type HtmlResult = StdResult<Html<String>, (StatusCode, String)>;

pub fn is_stopped() -> Result<bool> {
    let dir = current_exe()?;
    let dir = dir
        .parent()
        .ok_or_else(|| Box::new(HttpError(StatusCode::INTERNAL_SERVER_ERROR, None)))?;
    log::debug!("current work dir {}", dir.display());
    Ok(Path::new(".stop").exists())
}
pub fn check_permission<P: AsRef<Path>>(file: P) -> Result<()> {
    match fs::metadata(file)?.permissions().mode() & 0o777 {
        0o400 | 0o600 => Ok(()),
        v => Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some(format!("file permission is too open({:#o})", v)),
        ))),
    }
}
pub fn parse_toml<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> Result<T> {
    let cfg = file.as_ref();
    log::debug!("load configuration from {}", cfg.display());
    check_permission(cfg)?;
    let it: T = toml::from_str(&fs::read_to_string(cfg)?)?;
    Ok(it)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key(pub Vec<u8>);

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", BASE64.encode(&self.0))
    }
}
impl FromStr for Key {
    type Err = Base64DecodeError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut buf = vec![0; BASE64.decode_len(s.len())?];
        BASE64
            .decode_mut(s.as_bytes(), &mut buf)
            .map_err(|x| x.error)?;
        Ok(Self(buf))
    }
}

pub trait Jwt {
    fn sign<A: Into<String>, P: Serialize>(
        &self,
        issuer: &str,
        subject: &str,
        audiences: Vec<A>,
        ttl: Duration,
        payload: Option<P>,
    ) -> impl Future<Output = Result<String>>;
    fn verify<P: DeserializeOwned>(
        &self,
        token: &str,
        issuer: &str,
        audience: &str,
    ) -> impl Future<Output = Result<(String, Option<P>)>>;
}

pub trait SecretBox {
    fn encrypt(&self, plain: &[u8]) -> impl Future<Output = Result<(Vec<u8>, Vec<u8>)>>;
    fn decrypt(
        &self,
        cipher: &[u8],
        associated_data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>>>;
}

pub trait PasswordHashing {
    fn sign(&self, password: &str) -> impl Future<Output = Result<String>>;
    fn verify(&self, hashed: &str, password: &str) -> impl Future<Output = Result<()>>;
}

pub struct Loquat {
    aes: loquat_v1::aes_client::AesClient<GrpcClientChannel>,
    jwt: loquat_v1::jwt_client::JwtClient<GrpcClientChannel>,
    argon2: loquat_v1::argon2_client::Argon2Client<GrpcClientChannel>,
}
impl Loquat {
    pub fn new(channel: GrpcClientChannel) -> Self {
        Self {
            aes: loquat_v1::aes_client::AesClient::new(channel.clone()),
            jwt: loquat_v1::jwt_client::JwtClient::new(channel.clone()),
            argon2: loquat_v1::argon2_client::Argon2Client::new(channel),
        }
    }
}
pub struct Dahlia {
    pub enforcer: rbac_v1::enforcer_client::EnforcerClient<GrpcClientChannel>,
}
impl Dahlia {
    pub fn new(channel: GrpcClientChannel) -> Self {
        Self {
            enforcer: rbac_v1::enforcer_client::EnforcerClient::new(channel),
        }
    }
}
pub struct Marigold {
    pub wechat_pay: wechat_pay_v1::we_chat_pay_client::WeChatPayClient<GrpcClientChannel>,
}

impl Marigold {
    pub fn new(channel: GrpcClientChannel) -> Self {
        Self {
            wechat_pay: wechat_pay_v1::we_chat_pay_client::WeChatPayClient::new(channel),
        }
    }
}

pub fn current_user() -> Result<String> {
    let it = nix::unistd::User::from_uid(nix::unistd::getuid())?.ok_or_else(|| {
        Box::new(HttpError(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("could't detect current username".to_string()),
        ))
    })?;
    Ok(it.name)
}

pub fn hostname() -> Result<String> {
    let it = nix::unistd::gethostname()?.into_string().map_err(|_| {
        Box::new(HttpError(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("could't detect hostname".to_string()),
        ))
    })?;
    Ok(it)
}

pub fn shell<P: AsRef<Path>, A: Into<String>>(
    working_dir: P,
    command: &str,
    args: Vec<A>,
) -> Result<String> {
    // let args: Vec<String> = args.into_iter().map(|x| x.into()).collect();
    // let output = Command::new("/use/bin/bash")
    //     .arg("-lc")
    //     .arg(format!("{} {}", command, args.join(" ")))
    //     .output()?;
    let mut command = Command::new(command);
    command.current_dir(working_dir);
    for it in args.into_iter() {
        let it: String = it.into();
        command.arg(&it);
    }
    let output = command.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if output.status.success() {
        return Ok(stdout);
    }
    Err(Box::new(HttpError(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(stderr),
    )))
}
