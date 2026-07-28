use std::fmt::Display;
use std::ops::DerefMut;

use juniper::{FieldResult, ScalarValue, graphql_object};
use portal::graphql::{Succeeded, user::email as email_user_api};

use super::context::Context;

pub struct Mutation;

#[graphql_object]
#[graphql(
    context = Context,
    scalar = S: ScalarValue + Display,
)]
impl Mutation {
    async fn sign_up_by_email<S: ScalarValue + Display>(
        req: email_user_api::SignUp,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();

        req.execute(
            &ctx.session,
            db,
            &ctx.state.queue,
            &ctx.state.loquat,
            &ctx.state.loquat,
        )
        .await?;
        Ok(Succeeded::default())
    }
}
