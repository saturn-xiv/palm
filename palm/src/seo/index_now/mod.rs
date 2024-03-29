pub mod ping;

use askama::Template;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use validator::Validate;

// https://www.indexnow.org/documentation
#[derive(GraphQLObject, Template, Validate, Serialize, Deserialize, Default)]
#[template(path = "index-now/verify.txt", escape = "none")]
#[graphql(name = "IndexNowSiteVerification")]
pub struct SiteVerification {
    #[validate(length(min = 1, max = 127))]
    pub key: String,
}
