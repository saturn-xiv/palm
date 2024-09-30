use juniper::graphql_object;

use super::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    fn user_sign_in_by_email(_context: &Context) -> &str {
        ""
    }
}
