use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(GraphQLObject)]
#[graphql(name = "IndexNowSiteOwnershipVerifying")]
#[derive(Serialize, Validate, Deserialize, Debug, Clone)]
pub struct SiteOwnershipVerifying {
    #[validate(length(min = 1, max = 127))]
    pub key: String,
}
