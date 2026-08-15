use std::fmt::Display;
use std::ops::DerefMut;

use chrono_tz::TZ_VARIANTS;
use juniper::{FieldResult, ScalarValue, graphql_object};
use portal::graphql::{
    Page, Succeeded, currency as currency_api, locale as locale_api,
    user::{self as user_api, email as email_user_api},
};

use super::super::{BUILD_TIME, GIT_VERSION};
use super::context::Context;

pub struct Query;

#[graphql_object]
#[graphql(context = Context, scalar = S: ScalarValue + Display)]
impl Query {
    fn api_version() -> &'static str {
        GIT_VERSION
    }
    fn build_time() -> &'static str {
        BUILD_TIME
    }

    fn timezones() -> Vec<String> {
        TZ_VARIANTS.iter().map(|x| x.name().to_string()).collect()
    }

    async fn forgot_password_for_email_user<S: ScalarValue + Display>(
        email: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = email_user_api::ForgotPassword {
            email: email.trim().to_lowercase(),
        };
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        form.execute(db, &ctx.state.queue, &ctx.state.loquat)
            .await?;
        Ok(Succeeded::default())
    }
    async fn unlock_for_email_user<S: ScalarValue + Display>(
        email: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = email_user_api::Unlock {
            email: email.trim().to_lowercase(),
        };
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        form.execute(db, &ctx.state.queue, &ctx.state.loquat)
            .await?;
        Ok(Succeeded::default())
    }
    async fn confirm_for_email_user<S: ScalarValue + Display>(
        email: String,
        ctx: &Context,
    ) -> FieldResult<Succeeded, S> {
        let form = email_user_api::Confirm {
            email: email.trim().to_lowercase(),
        };
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        form.execute(db, &ctx.state.queue, &ctx.state.loquat)
            .await?;
        Ok(Succeeded::default())
    }

    async fn refresh(ctx: &Context) -> FieldResult<user_api::RefreshResponse> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let mut cache = ctx.state.cache.get()?;
        let reply = user_api::RefreshResponse::new(
            &ctx.session,
            db,
            &mut cache,
            &ctx.state.loquat,
            &ctx.state.dahlia,
            GIT_VERSION,
        )
        .await?;
        Ok(reply)
    }

    fn index_currency(ctx: &Context) -> FieldResult<Vec<currency_api::Item>> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let reply = currency_api::index(db)?;
        Ok(reply)
    }

    fn locale_by_lang(lang: String, ctx: &Context) -> FieldResult<Vec<locale_api::Item>> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let reply = locale_api::by_lang(db, &lang)?;
        Ok(reply)
    }

    async fn index_locale(page: Page, ctx: &Context) -> FieldResult<locale_api::Index> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let mut cache = ctx.state.cache.get()?;
        let reply = locale_api::Index::new(
            &ctx.session,
            db,
            &mut cache,
            &ctx.state.dahlia,
            &ctx.state.loquat,
            &page,
        )
        .await?;
        Ok(reply)
    }
}
