pub mod response;

use std::any::type_name;
use std::fmt::Debug;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::result::Result as StdResult;

use hyper::StatusCode;
use opensearch::{
    CountParts, DeleteByQueryParts, DeleteParts, Error as OpenSearchError, IndexParts, OpenSearch,
    http::{
        Url,
        response::Response,
        transport::{SingleNodeConnectionPool, TransportBuilder},
    },
    indices::{IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts},
    models::InfoResponse,
    params::Refresh,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
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
        let _: response::index_create::Item = Self::response(res).await?;
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
    pub async fn search_document<T: Debug + Clone + Serialize + DeserializeOwned>(
        &self,
        query: Value,
    ) -> OpenSearchResult<response::document_search::Item<T>> {
        let name = self.index_name::<T>();
        log::debug!("search document {name}");
        let res = self
            .db
            .search(opensearch::SearchParts::Index(&[&name]))
            .body(query)
            .send()
            .await?;

        let it = Self::response(res).await?;
        Ok(it)
    }

    pub async fn count_document_by_query<T>(&self, query: Value) -> OpenSearchResult<usize> {
        let name = self.index_name::<T>();
        log::debug!("count document {name} by {query}");
        let res = self
            .db
            .count(CountParts::Index(&[&name]))
            .body(query)
            .send()
            .await?;
        let body: response::document_count::Item = Self::response(res).await?;
        Ok(body.count)
    }

    pub async fn delete_document_by_id<T>(&self, id: &str) -> OpenSearchResult<()> {
        let name = self.index_name::<T>();
        log::debug!("delete document {name} by {id}");
        let res = self
            .db
            .delete(DeleteParts::IndexId(&name, id))
            .refresh(Refresh::True)
            .send()
            .await?;
        let _: response::document_delete_by_id::Item = Self::response(res).await?;
        Ok(())
    }

    pub async fn delete_document_by_query<T>(&self, query: Value) -> OpenSearchResult<()> {
        let name = self.index_name::<T>();
        log::debug!("delete document {name} by {query}");
        let res = self
            .db
            .delete_by_query(DeleteByQueryParts::Index(&[&name]))
            .body(query)
            .refresh(true)
            .send()
            .await?;
        let _: response::document_delete_by_query::Item = Self::response(res).await?;
        Ok(())
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

    async fn response<T: DeserializeOwned>(res: Response) -> OpenSearchResult<T> {
        let status = res.status_code();
        if status != StatusCode::OK {
            let body = res.text().await?;
            log::error!("{status} {body}");
            return Err(IoError::from(IoErrorKind::InvalidData).into());
        }
        let it: T = res.json().await?;
        Ok(it)
    }
}
