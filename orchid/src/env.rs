use std::path::Path;

use hyper::StatusCode;
use palm::{
    cache::redis::Config as Redis, parser::from_toml, session::Session,
    wechat::Config as WechatConfig, HttpError, Result,
};
use serde::{Deserialize, Serialize};
use tonic::Request;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub redis: Redis,

    pub clients: Vec<String>,
}

impl Config {
    pub const AUDIENCE: &'static str = env!("CARGO_PKG_NAME");

    pub fn wechat(&self, app_id: &str) -> Result<WechatConfig> {
        let file = Path::new("wechat").join(app_id).with_extension("toml");
        let it = from_toml(&file)?;
        Ok(it)
    }
    pub fn verify<T>(&self, req: &Request<T>) -> Result<String> {
        // FIXME
        let ss = Session::new(req);
        if let Some(ref _token) = ss.token {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("unknown client ".to_string()),
            )));
        }
        Err(Box::new(HttpError(StatusCode::UNAUTHORIZED, None)))
    }
}
