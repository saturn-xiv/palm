use juniper::GraphQLInputObject;

use phlox::{Result, orm::postgresql::Connection as Db};
use serde::{Deserialize, Serialize};

#[derive(GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[graphql(name="WechatMiniProgramUserSignUpRequest" description = "Register a WeChat mini-program user")]
pub struct SignUp {
    pub username: String,
}

impl SignUp {
    pub async fn execute(&self, _db: &mut Db) -> Result<()> {
        // TODO
        Ok(())
    }
}
