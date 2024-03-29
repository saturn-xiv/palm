use askama::Template;
use hyper::StatusCode;
use juniper::GraphQLObject;
use quick_xml::se::to_string as to_xml_string;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::super::{Error, HttpError};

#[derive(GraphQLObject, Template, Validate, Serialize, Deserialize, Default)]
#[graphql(name = "BaiduSiteVerification")]
#[template(path = "baidu/verify.html", escape = "none")]

pub struct SiteVerification {
    #[validate(length(min = 1, max = 127))]
    pub code: String,
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

    debug!("ping baidu:\n{body}");
    let cli = reqwest::Client::new();
    let res = cli
        .get("https://ping.baidu.com/ping/RPC2")
        .header(CONTENT_TYPE, "text/xml;charset='utf-8")
        .body(body)
        .send()
        .await?;

    debug!("{:#?}", res);
    if res.status() == reqwest::StatusCode::OK {
        return Ok(());
    }
    Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)))
}
