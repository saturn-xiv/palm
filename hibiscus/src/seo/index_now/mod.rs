pub mod ping;

use askama::Template;
use serde::{Deserialize, Serialize};
use validator::Validate;

// https://www.indexnow.org/documentation
#[derive(Template, Validate, Serialize, Deserialize, Default)]
#[template(path = "index-now/verify.txt", escape = "none")]
pub struct SiteVerification {
    #[validate(length(min = 1, max = 127))]
    pub key: String,
}
