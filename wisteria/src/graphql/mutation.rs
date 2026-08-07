use std::fmt::Display;
use std::ops::DerefMut;

use juniper::{FieldResult, ScalarValue, graphql_object};
use portal::graphql::{
    Succeeded, locale as locale_api, user as user_api, user::email as email_user_api,
};

use super::context::Context;

pub struct Mutation;

#[graphql_object]
#[graphql(
    context = Context,
    scalar = S: ScalarValue + Display,
)]
impl Mutation {
    async fn set_password_for_email_user<S: ScalarValue + Display>(
        id: i32,
        password: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = email_user_api::SetPassword { id, password };
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
    async fn sign_in_by_email<S: ScalarValue + Display>(
        email: String,
        password: String,
        ctx: &Context,
    ) -> FieldResult<user_api::SignInResponse, S> {
        let form = email_user_api::SignIn {
            email: email.trim().to_lowercase(),
            password,
        };
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let mut cache = ctx.state.cache.get()?;
        let it = form
            .execute(
                &ctx.session,
                db,
                &mut cache,
                &ctx.state.dahlia,
                &ctx.state.loquat,
                &ctx.state.loquat,
            )
            .await?;
        Ok(it)
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

    async fn set_locale<S: ScalarValue + Display>(
        lang: String,
        code: String,
        message: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = locale_api::Set {
            lang,
            code,
            message,
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
        )
        .await?;
        Ok(Succeeded::default())
    }
    async fn destroy_locale<S: ScalarValue + Display>(
        id: i32,

        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let mut cache = ctx.state.cache.get()?;

        locale_api::destroy(
            &ctx.session,
            db,
            &mut cache,
            &ctx.state.dahlia,
            &ctx.state.loquat,
            id,
        )
        .await?;
        Ok(Succeeded::default())
    }
}
