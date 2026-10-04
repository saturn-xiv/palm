use juniper::GraphQLObject;

use super::super::super::super::{
    Result,
    s3::seaweedfs::{Client, responses::ClusterStatus},
};

#[derive(Debug, Default, GraphQLObject)]
#[graphql(name = "SeaweedFSClusterStatus")]
pub struct Item {
    pub is_leader: bool,
    pub leader: String,
    pub max_volume_id: i32,
}

impl Item {
    pub async fn new(client: &Client) -> Result<Self> {
        let it = client.cluster_status().await?;
        Ok(it.into())
    }
}

impl From<ClusterStatus> for Item {
    fn from(it: ClusterStatus) -> Self {
        Self {
            is_leader: it.is_leader,
            leader: it.leader.clone(),
            max_volume_id: it.max_volume_id as i32,
        }
    }
}
