use juniper::GraphQLObject;
use petunia::{
    cache::{redis::ClusterConnection as Cache, Provider as CacheProvider},
    Result,
};

#[derive(GraphQLObject)]
#[graphql(name = "RedisStatus")]
pub struct Status {
    pub version: Vec<String>,
}

impl Status {
    pub fn new(ch: &mut Cache) -> Result<Self> {
        let it = Self {
            version: {
                let it = CacheProvider::version(ch)?;
                it.split("\r\n").map(|x| x.to_string()).collect()
            },
        };
        Ok(it)
    }
}
