use juniper::GraphQLObject;

use super::super::super::super::{Result, opensearch::Client};

#[derive(Debug, Default, GraphQLObject)]
#[graphql(name = "OpenSearchStatus")]
pub struct Item {
    pub version: String,
}

impl Item {
    pub async fn new(client: &Client) -> Result<Self> {
        let _ = client.info().await?;
        let it = Self {
            // TODO
            version: "".to_string(),
        };
        Ok(it)
    }
}
