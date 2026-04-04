use std::fmt::Display;
use std::ops::DerefMut;

use juniper::{FieldResult, ScalarValue, graphql_object};

use super::super::plugins::portal::graphql as portal;
use super::context::Context;

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql(
    context = Context,scalar = S: ScalarValue + Display,
)]
impl Mutation {
    pub async fn sign_up_by_wechat_mini_program_user<S: ScalarValue + Display>(
        form: portal::wechat_mini_program_user::SignUp,
        ctx: &Context,
    ) -> FieldResult<portal::Ok, S> {
        let mut db = ctx.db.get()?;
        let db = db.deref_mut();
        form.execute(db).await?;
        Ok(portal::Ok::default())
    }
}
