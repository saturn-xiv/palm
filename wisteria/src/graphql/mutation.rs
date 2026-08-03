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
    async fn set_password_for_email_user<S: ScalarValue + Display>(
        req: email_user_api::SetPassword,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = email_user_api::SetPassword {
            email: req.email.trim().to_lowercase(),
            ..req.clone()
        };
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let mut cache = ctx.state.cache.get()?;

        form.execute(
            &ctx.session,
            db,
            &mut cache,
            &ctx.state.dahlia,
            &ctx.state.loquat,
            &ctx.state.loquat,
        )
        .await?;
        Ok(Succeeded::default())
    }
    async fn sign_up_by_email<S: ScalarValue + Display>(
        req: email_user_api::SignUp,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = email_user_api::SignUp {
            name: req.name.trim().to_string(),
            email: req.email.trim().to_lowercase(),
            ..req.clone()
        };
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();

        form.execute(
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
