use askama::Template;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::super::{Error, HttpError};

#[derive(Template, Validate, Serialize, Deserialize)]
#[template(path = "google/verify.html", escape = "none")]

pub struct Request {
    #[validate(length(min = 1))]
    pub site_verify_code: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Profile {
    #[serde(rename = "site-verify-id")]
    #[validate(length(min = 1, max = 63))]
    pub site_verify_id: Option<String>,
    #[serde(rename = "re-captcha")]
    pub re_captcha: Option<ReCaptcha>,
}

// https://developers.google.com/recaptcha/docs/v3
#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct ReCaptcha {
    #[serde(rename = "site-key")]
    #[validate(length(min = 1, max = 127))]
    pub site_key: String,
    #[validate(length(min = 1, max = 127))]
    pub secret: String,
}

// https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap#addsitemap
pub async fn ping(home: &str) -> Result<(), Error> {
    let cli = reqwest::Client::new();
    let res = cli
        .get("https://www.google.com/ping")
        .query(&("sitemap", format!("{home}/sitemap.xml")))
        .send()
        .await?;

    debug!("{:#?}", res);
    if res.status() == reqwest::StatusCode::OK {
        return Ok(());
    }
    Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)))
}
