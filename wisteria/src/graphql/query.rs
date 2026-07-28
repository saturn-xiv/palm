use std::ops::DerefMut;

use juniper::{FieldResult, graphql_object};
use portal::graphql::locale as locale_api;

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

    fn index_locale_by_lang(lang: String, ctx: &Context) -> FieldResult<Vec<locale_api::Item>> {
        let mut db = ctx.state.db.get()?;
        let db = db.deref_mut();
        let reply = locale_api::by_lang(db, &lang)?;
        Ok(reply)
    }
}
