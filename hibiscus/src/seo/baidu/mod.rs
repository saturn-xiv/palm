pub mod ping;

use askama::Template;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(GraphQLObject, Template, Validate, Serialize, Deserialize, Default)]
#[graphql(name = "BaiduSiteVerification")]
#[template(path = "baidu/verify.html", escape = "none")]

pub struct SiteVerification {
    #[validate(length(min = 1, max = 127))]
    pub code: String,
    #[validate(length(min = 1, max = 127))]
    pub content: String,
}
