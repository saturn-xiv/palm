use juniper::graphql_object;
use palm::GIT_VERSION;

use super::context::Context;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn apiVersion(_context: &Context) -> &str {
        GIT_VERSION
    }
}
