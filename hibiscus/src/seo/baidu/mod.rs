pub mod ping;

use askama::Template;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Template, Validate, Serialize, Deserialize, Default)]
#[template(path = "baidu/verify.html", escape = "none")]

pub struct SiteVerification {
    #[validate(length(min = 1, max = 127))]
    pub code: String,
    #[validate(length(min = 1, max = 127))]
    pub content: String,
}
