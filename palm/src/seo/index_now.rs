use askama::Template;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::super::{Error, HttpError};

#[derive(Template, Validate, Serialize, Deserialize)]
#[template(path = "index-now/verify.txt", escape = "none")]

pub struct Request {
    #[validate(length(min = 1))]
    pub key: String,
}

// https://www.indexnow.org/documentation
#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Profile {
    #[validate(length(min = 1, max = 127))]
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub host: String,
    pub key: String,
    #[serde(rename = "urlList")]
    pub items: Vec<String>,
}

// https://www.indexnow.org/documentation
pub async fn ping(home: &str, key: &str, links: &[String]) -> Result<(), Error> {
    let limit = 10_000;
    let mut i = 0;
    loop {
        let begin = i * limit;
        if begin >= links.len() {
            break;
        }
        let end = {
            let j = begin + limit;
            if j > links.len() {
                links.len()
            } else {
                j
            }
        };

        let cli = reqwest::Client::new();
        let res = cli
            .post("https://api.indexnow.org/indexnow")
            .json(&Body {
                host: home.to_string(),
                key: key.to_string(),
                items: links[begin..end].to_vec(),
            })
            .send()
            .await?;
        debug!("{:#?}", res);
        if res.status() != reqwest::StatusCode::OK {
            return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
        }

        i += 1;
    }
    Ok(())
}
