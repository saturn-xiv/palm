use std::fmt::Display;
use std::ops::DerefMut;

use hyacinth::{password as parse_password, portal_v1};
use juniper::{FieldResult, ScalarValue, graphql_object};
use portal::graphql::{
    Succeeded, locale as locale_api,
    user::{self as user_api, email as email_user_api},
};

use super::super::GIT_VERSION;
use super::context::Context;

pub struct Mutation;

#[graphql_object]
#[graphql(context = Context, scalar = S: ScalarValue + Display)]
impl Mutation {
    async fn set_password_for_email_user<S: ScalarValue + Display>(
        id: i32,
        password: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = email_user_api::SetPassword {
            id,
            password: {
                parse_password!(it, portal_v1::Password, &password);
                let it = it.payload();
                it.to_string()
            },
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

    async fn reset_password_for_email_user<S: ScalarValue + Display>(
        token: String,
        password: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = email_user_api::ResetPassword {
            token,
            password: {
                parse_password!(it, portal_v1::Password, &password);
                let it = it.payload();
                it.to_string()
            },
        };
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();

        form.execute(&ctx.session, db, &ctx.state.loquat, &ctx.state.loquat)
            .await?;
        Ok(Succeeded::default())
    }
    async fn unlock_for_email_user<S: ScalarValue + Display>(
        token: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        email_user_api::unlock(&ctx.session, db, &ctx.state.loquat, &token).await?;
        Ok(Succeeded::default())
    }
    async fn confirm_for_email_user<S: ScalarValue + Display>(
        token: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        email_user_api::confirm(&ctx.session, db, &ctx.state.loquat, &token).await?;
        Ok(Succeeded::default())
    }
    async fn sign_in_by_email<S: ScalarValue + Display>(
        email: String,
        password: String,
        ctx: &Context,
    ) -> FieldResult<user_api::SignInResponse, S> {
        let form = email_user_api::SignIn {
            email: email.trim().to_lowercase(),
            password: {
                parse_password!(it, portal_v1::Password, &password);
                let it = it.payload();
                it.to_string()
            },
        };
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let mut cache = ctx.state.cache.get()?;
        let it = form
            .execute(
                &ctx.session,
                db,
                &mut cache,
                (&ctx.state.dahlia, &ctx.state.loquat, &ctx.state.loquat),
                GIT_VERSION,
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
            password: {
                parse_password!(it, portal_v1::Password, &req.password);
                let it = it.payload();
                it.to_string()
            },
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
