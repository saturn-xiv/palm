use askama::Template;
use hyper::StatusCode;
use quick_xml::se::to_string as to_xml_string;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::super::{Error, HttpError};

#[derive(Template, Validate, Serialize, Deserialize)]
#[template(path = "baidu/verify.html", escape = "none")]

pub struct Request {
    #[validate(length(min = 1))]
    pub site_verify_code: String,
    #[validate(length(min = 1))]
    pub site_verify_content: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Profile {
    #[serde(rename = "site-verify")]
    pub site_verify: Option<SiteVerify>,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct SiteVerify {
    #[validate(length(min = 1, max = 127))]
    pub id: String,
    #[validate(length(min = 1, max = 127))]
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@rss")]
    pub rss: String,
    #[serde(rename = "@home")]
    pub home: String,
}

// https://help.baidu.com/question?prod_id=99&class=0&id=3046
pub async fn ping(home: &str, title: &str, lang: &str) -> Result<(), Error> {
    let body = to_xml_string(&Body {
        title: title.to_string(),
        home: home.to_string(),
        rss: format!("{home}/rss/{lang}.xml"),
    })?;

    let cli = reqwest::Client::new();
    let res = cli
        .get("https://ping.baidu.com/ping/RPC2")
        .header(CONTENT_TYPE, "text/xml")
        .body(body)
        .send()
        .await?;

    debug!("{:#?}", res);
    if res.status() == reqwest::StatusCode::OK {
        return Ok(());
    }
    Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)))
}
