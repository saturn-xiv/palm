use camelia::graphql as camelia_graphql;
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object]
impl Mutation {
    fn signIn(
        _request: camelia_graphql::user::SignInRequest,
    ) -> FieldResult<camelia_graphql::user::SignInResponse> {
        // let it = request.execute()?;
        // Ok(it)
        todo!()
    }
}
