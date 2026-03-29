pub mod response;

use std::{any::type_name, result::Result as StdResult};

use opensearch::{
    Error as OpenSearchError, IndexParts, OpenSearch,
    http::{
        Url,
        transport::{SingleNodeConnectionPool, TransportBuilder},
    },
    indices::{IndicesCreateParts, IndicesDeleteParts},
    models::InfoResponse,
};
use serde::{Deserialize, Serialize};

pub use serde_json::Value;

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
            host: "http://localhost:9200".to_string(),
            namespace: None,
        }
    }
}

impl Node {
    pub fn single(&self) -> OpenSearchResult<Client> {
        log::info!("open OpenSearch {}", self.host);
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
    pub async fn create_index<T>(&self, config: Value) -> OpenSearchResult<()> {
        let name = self.index_name::<T>();
        self.db
            .indices()
            .create(IndicesCreateParts::Index(&name))
            .body(config)
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_index<T>(&self) -> OpenSearchResult<()> {
        let name = self.index_name::<T>();
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
        self.db
            .index(IndexParts::Index(&name))
            .body(item)
            .send()
            .await?;
        Ok(())
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
        s.to_lowercase().replace("::", "-")
    }
}
