use juniper::GraphQLObject;
use petunia::{s3::Client, Result};

#[derive(GraphQLObject)]
#[graphql(name = "MinioStatus")]
pub struct Status {
    pub buckets: Vec<String>,
}

impl Status {
    pub async fn new(cli: &Client) -> Result<Self> {
        let it = Self {
            buckets: cli.list_buckets().await?,
        };
        Ok(it)
    }
}
