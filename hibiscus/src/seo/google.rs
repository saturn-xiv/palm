use askama::Template;
use hyper::StatusCode;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::super::{Error, HttpError};

#[derive(GraphQLObject, Template, Validate, Serialize, Deserialize, Default)]
#[graphql(name = "GoogleSiteVerification")]
#[template(path = "google/verify.html", escape = "none")]

pub struct SiteVerification {
    #[validate(length(min = 1, max = 127))]
    pub code: String,
}

// https://developers.google.com/recaptcha/docs/v3
#[derive(GraphQLObject, Validate, Serialize, Deserialize, Debug, Default)]
#[graphql(name = "GoogleReCaptcha")]
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
