pub mod attachments;
pub mod log;
pub mod user;

use chrono::{NaiveDateTime, Utc};
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(name = "Succeed")]
pub struct Succeed {
    pub created_at: NaiveDateTime,
}

impl Default for Succeed {
    fn default() -> Self {
        Self {
            created_at: Utc::now().naive_utc(),
        }
    }
}
