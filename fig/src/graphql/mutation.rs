use camelia::graphql as camelia_graphql;
use juniper::{graphql_object, FieldResult};

use super::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    fn signIn(
        context: &Context,
        user: String,
        password: String,
        ttl: Option<i32>,
    ) -> FieldResult<camelia_graphql::user::SignInResponse> {
        let request = camelia_graphql::user::SignInRequest {
            user,
            password,
            ttl,
        };
        // let it = request.execute()?;
        // Ok(it)
        todo!()
    }
    async fn signUp(
        context: &Context,
        real_name: String,
        nickname: String,
        email: String,
        password: String,
        lang: String,
        timezone: String,
    ) -> FieldResult<camelia_graphql::Succeed> {
        // camelia_graphql::user::SignInRequest { user, password };
        // let it = request.execute()?;
        // Ok(it)
        todo!()
    }
}
