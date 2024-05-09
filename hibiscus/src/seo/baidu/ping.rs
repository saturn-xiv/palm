use hyper::StatusCode;
use palm::{Error, HttpError, XML_HEADER};
use quick_xml::se::to_string as to_xml_string;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

// https://help.baidu.com/question?prod_id=99&class=0&id=3046
// https://github.com/CleanSky/PHP/blob/master/phpLearning/google_baidu_shoulu.php
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "methodCall")]
pub struct MethodCall {
    #[serde(rename = "methodName")]
    pub method_name: MethodName,
    #[serde(rename = "params")]
    params: Params,
}

impl MethodCall {
    pub fn new(home: &str, title: &str, lang: &str) -> Self {
        Self {
            method_name: MethodName {
                body: "weblogUpdates.extendedPing".to_string(),
            },
            params: Params {
                items: vec![
                    Param {
                        value: Value {
                            body: title.to_string(),
                        },
                    },
                    Param {
                        value: Value {
                            body: home.to_string(),
                        },
                    },
                    Param {
                        value: Value {
                            body: format!("{home}/{lang}/"),
                        },
                    },
                    Param {
                        value: Value {
                            body: format!("{home}/{lang}/rss.xml"),
                        },
                    },
                ],
            },
        }
    }

    pub async fn ping(&self) -> Result<(), Error> {
        let body = format!("{}\n{}", XML_HEADER, to_xml_string(self)?);
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "param")]
    pub items: Vec<Param>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    #[serde(rename = "value")]
    pub value: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    #[serde(rename = "$text")]
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MethodName {
    #[serde(rename = "$text")]
    pub body: String,
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
