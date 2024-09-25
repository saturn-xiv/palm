use juniper::graphql_object;
use petunia::GIT_VERSION;

use super::context::Context;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn api_version(_context: &Context) -> &str {
        GIT_VERSION
    }
}
