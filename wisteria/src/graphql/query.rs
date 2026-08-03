use std::ops::DerefMut;

use juniper::{FieldResult, graphql_object};
use portal::graphql::{Page, currency as currency_api, locale as locale_api};

use super::super::{BUILD_TIME, GIT_VERSION};
use super::context::Context;

pub struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        GIT_VERSION
    }
    fn build_time() -> &'static str {
        BUILD_TIME
    }

    fn index_currency(ctx: &Context) -> FieldResult<Vec<currency_api::Item>> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let reply = currency_api::index(db)?;
        Ok(reply)
    }

    fn index_locale_by_lang(lang: String, ctx: &Context) -> FieldResult<Vec<locale_api::Item>> {
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
