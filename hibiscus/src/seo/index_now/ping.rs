use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use super::super::super::{HttpError, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub host: String,
    pub key: String,
    #[serde(rename = "urlList")]
    pub items: Vec<String>,
}

impl Request {
    pub fn new(home: &str, key: &str, links: &[String]) -> Vec<Self> {
        let limit = 10_000;

        let mut items = Vec::new();
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

            items.push(Self {
                host: home.to_string(),
                key: key.to_string(),
                items: links[begin..end].to_vec(),
            });

            i += 1;
        }
        items
    }

    pub async fn ping(&self) -> Result<()> {
        let cli = reqwest::Client::new();
        let res = cli
            .post("https://api.indexnow.org/indexnow")
            .json(self)
            .send()
            .await?;
        debug!("{:#?}", res);
        if res.status() != reqwest::StatusCode::OK {
            return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
        }
        Ok(())
    }
}
