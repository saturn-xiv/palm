use juniper::graphql_object;

use super::super::{BUILD_TIME, GIT_VERSION};
use super::context::Context;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn api_version() -> String {
        format!("{}({})", GIT_VERSION, BUILD_TIME)
    }
}
