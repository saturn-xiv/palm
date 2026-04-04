pub mod wechat_mini_program_user;

use juniper::GraphQLObject;
use phlox::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(GraphQLObject, Serialize, Deserialize)]
#[graphql(description = "Succeed")]
#[serde(rename_all = "camelCase")]
pub struct Ok {
    pub created_at: NaiveDateTime,
}

impl Default for Ok {
    fn default() -> Self {
        Self {
            created_at: Utc::now().naive_utc(),
        }
    }
}
