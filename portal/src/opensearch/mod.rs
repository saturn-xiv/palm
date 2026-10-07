pub mod response;

use std::any::type_name;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::result::Result as StdResult;

use hyper::StatusCode;
use opensearch::{
    Error as OpenSearchError, IndexParts, OpenSearch,
    http::{
        Url,
        transport::{SingleNodeConnectionPool, TransportBuilder},
    },
    indices::{IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts},
    models::InfoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub type OpenSearchResult<T> = StdResult<T, OpenSearchError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(default = "node_default_host")]
    pub host: String,
    #[serde(default)]
    pub namespace: Option<String>,
}

fn node_default_host() -> String {
    "http://localhost:9200".to_string()
}

impl Default for Node {
    fn default() -> Self {
        Self {
            host: node_default_host(),
            namespace: None,
        }
    }
}

impl Node {
    pub fn single(&self) -> OpenSearchResult<Client> {
        log::debug!("open OpenSearch {}", self.host);
        let transport =
            TransportBuilder::new(SingleNodeConnectionPool::new(Url::parse(&self.host)?))
                .disable_proxy()
                .build()?;

        let it = Client {
            db: OpenSearch::new(transport),
            namespace: self.namespace.clone(),
        };
        Ok(it)
    }
}

pub struct Client {
    db: OpenSearch,
    namespace: Option<String>,
}

impl Client {
    // https://docs.opensearch.org/latest/api-reference/index-apis/create-index/
    // https://docs.opensearch.org/latest/mappings/supported-field-types/index/
    pub async fn create_index<T>(
        &self,
        settings: Option<Value>,
        mappings: Option<Value>,
    ) -> OpenSearchResult<()> {
        let name = self.index_name::<T>();
        log::warn!("create index {name}");
        let res = self
            .db
            .indices()
            .create(IndicesCreateParts::Index(&name))
            .body({
                let mut it = json!({});
                if let Some(v) = settings {
                    it["settings"] = v;
                }
                if let Some(v) = mappings {
                    it["mappings"] = v;
                }
                it
            })
            .send()
            .await?;

        let status = res.status_code();
        if status != StatusCode::OK {
            let body = res.text().await?;
            log::error!("{status} {body}");
            return Err(IoError::from(IoErrorKind::InvalidData).into());
        }
        let _: response::create_index::Item = res.json().await?;
        Ok(())
    }

    pub async fn delete_index<T>(&self) -> OpenSearchResult<()> {
        let name = self.index_name::<T>();
        log::warn!("delete index {name}");
        self.db
            .indices()
            .delete(IndicesDeleteParts::Index(&[&name]))
            .send()
            .await?;
        Ok(())
    }

    // https://docs.opensearch.org/latest/api-reference/search-apis/search/
    pub async fn search_document<T: Serialize>(&self, query: Value) -> OpenSearchResult<Vec<T>> {
        let name = self.index_name::<T>();
        log::debug!("search document {name}");
        let res = self
            .db
            .search(opensearch::SearchParts::Index(&[&name]))
            .body(serde_json::json!({"query": query}))
            .send()
            .await?;

        let _body: response::document_search::Item = res.json().await?;
        let items = Vec::new();
        // TODO
        Ok(items)
    }

    pub async fn index_document<T: Serialize>(&self, item: &T) -> OpenSearchResult<()> {
        let name = self.index_name::<T>();
        log::debug!("index document {name}");
        self.db
            .index(IndexParts::Index(&name))
            .body(item)
            .send()
            .await?;
        Ok(())
    }

    // https://docs.opensearch.org/latest/api-reference/index-apis/exists/
    pub async fn index_exists<T>(&self) -> OpenSearchResult<bool> {
        let name = self.index_name::<T>();
        log::debug!("check index {name} exists");
        let res = self
            .db
            .indices()
            .exists(IndicesExistsParts::Index(&[name.as_str()]))
            .send()
            .await?;
        Ok(res.status_code() == StatusCode::OK)
    }

    pub async fn info(&self) -> OpenSearchResult<InfoResponse> {
        let res: InfoResponse = self.db.info().send().await?.json().await?;
        Ok(res)
    }

    // https://docs.opensearch.org/latest/api-reference/index-apis/create-index/#index-naming-restrictions
    pub fn index_name<T>(&self) -> String {
        let n = type_name::<T>();
        let s = match self.namespace {
            Some(ref it) => format!("{}.{}", it, n),
            None => n.to_string(),
        };
        s.to_lowercase().replace("::", ".")
    }
}
