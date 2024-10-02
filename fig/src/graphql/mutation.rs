use daffodil::graphql::user::email as daffodil_user_by_email;
use juniper::graphql_object;

use super::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    fn user_sign_in_by_email(_context: &Context, _form: daffodil_user_by_email::SignIn) -> &str {
        ""
    }
    fn user_sign_up_by_email(_context: &Context, _form: daffodil_user_by_email::SignUp) -> &str {
        ""
    }
}
