pub mod mini_program;
pub mod oauth2;
pub mod webhook;

use hyper::StatusCode;
use reqwest::Response;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::{HttpError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errcode")]
    pub code: usize,
    #[serde(rename = "errmsg")]
    pub message: String,
}

pub async fn to_json<T: DeserializeOwned>(res: Response) -> Result<T> {
    let status = res.status();
    if !status.is_success() {
        let it: ErrorResponse = res.json().await?;
        log::error!("{:?}", it);
        return Err(Box::new(HttpError(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(it.message),
        )));
    }
    Ok(res.json().await?)
}
