use hyper::StatusCode;
use opensearch::{
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    OpenSearch,
};
use serde::{Deserialize, Serialize};

use super::{HttpError, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    #[serde(rename = "index-prefix")]
    pub index_prefix: String,
}

impl Config {
    pub fn open(&self) -> Result<OpenSearch> {
        let url = {
            let mut it = Url::parse(&format!("http://{}:{}", self.host, self.port))?;
            it.set_username(&self.user)
                .map_err(|()| Box::new(HttpError(StatusCode::BAD_REQUEST, None)))?;
            it.set_password(Some(&self.password))
                .map_err(|()| Box::new(HttpError(StatusCode::BAD_REQUEST, None)))?;
            it
        };
        let pool = SingleNodeConnectionPool::new(url);
        let transport = TransportBuilder::new(pool).disable_proxy().build()?;
        let client = OpenSearch::new(transport);
        Ok(client)
    }
}