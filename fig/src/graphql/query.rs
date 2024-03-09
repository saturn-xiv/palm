use juniper::graphql_object;
use palm::GIT_VERSION;

pub struct Query;

#[graphql_object]
impl Query {
    fn apiVersion() -> &str {
        GIT_VERSION
    }
}
