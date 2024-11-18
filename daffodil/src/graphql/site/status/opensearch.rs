use juniper::GraphQLObject;
use opensearch::OpenSearch;
use petunia::Result;

#[derive(GraphQLObject)]
#[graphql(name = "OpenSearchStatus")]
pub struct Status {
    pub plugins: Vec<String>,
}

impl Status {
    pub async fn new(search: &OpenSearch) -> Result<Self> {
        let it = Self {
            plugins: {
                let res = search.cat().plugins().send().await?;
                let it = res.text().await?;
                it.split("\n").map(|x| x.to_string()).collect()
            },
        };
        Ok(it)
    }
}
