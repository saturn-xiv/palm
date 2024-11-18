use std::ops::Deref;

use carnation::graphql::page as cms_page;
use chrono::Duration;
use chrono_tz::TZ_VARIANTS;
use daffodil::graphql::{
    attachment as daffodil_attachment, category as daffodil_category,
    currency as daffodil_currency, leave_word as daffodil_leave_word, locale as daffodil_locale,
    log as daffodil_log, menu as daffodil_menu, session as daffodil_session, site as daffodil_site,
    tag as daffodil_tag, user::email as daffodil_user_by_email,
};
use hyacinth::graphql as hyacinth_graphql;
use juniper::{graphql_object, FieldResult};
use petunia::{
    graphql::{Pager, Succeed},
    themes::{Layout, Menu},
    GIT_VERSION,
};
use wisteria::graphql as wisteria_graphql;

use super::context::Context;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn api_version(_context: &Context) -> &str {
        GIT_VERSION
    }
    // ------------------------------------------------------------------------
    fn currencies(context: &Context) -> FieldResult<Vec<daffodil_currency::Item>> {
        let db = context.postgresql.deref();
        let items = daffodil_currency::Item::all(db)?;
        Ok(items)
    }
    fn timezones(_context: &Context) -> FieldResult<Vec<String>> {
        let items = TZ_VARIANTS.iter().map(|x| x.to_string()).collect();
        Ok(items)
    }
    // ------------------------------------------------------------------------
    async fn refresh(context: &Context) -> FieldResult<daffodil_site::Refresh> {
        let db = context.postgresql.deref();
        let secrets = context.secrets.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let it =
            daffodil_site::Refresh::new(&context.session, db, jwt, secrets.clone(), enf).await?;
        Ok(it)
    }
    // ------------------------------------------------------------------------
    fn get_email_user_profile(
        context: &Context,
    ) -> FieldResult<daffodil_user_by_email::GetProfile> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let it = daffodil_user_by_email::GetProfile::new(&context.session, db, jwt)?;
        Ok(it)
    }
    async fn send_confirm_email_for_user(context: &Context, user: String) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::Email {
            user: user.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let queue = context.rabbitmq.deref();
        let jwt = context.jwt.deref();
        form.confirm(db, jwt, queue).await?;
        Ok(Succeed::default())
    }
    async fn send_unlock_email_for_user(context: &Context, user: String) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::Email {
            user: user.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let queue = context.rabbitmq.deref();
        let jwt = context.jwt.deref();
        form.unlock(db, jwt, queue).await?;
        Ok(Succeed::default())
    }
    async fn send_forgot_password_email_for_user(
        context: &Context,
        user: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::Email {
            user: user.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let queue = context.rabbitmq.deref();
        let jwt = context.jwt.deref();
        form.forgot_password(db, jwt, queue).await?;
        Ok(Succeed::default())
    }
    async fn index_email_user(
        context: &Context,
        pager: Pager,
    ) -> FieldResult<daffodil_user_by_email::List> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let res = daffodil_user_by_email::List::new(&context.session, db, jwt, enf, &pager).await?;
        Ok(res)
    }

    // ------------------------------------------------------------------------
    fn index_log(context: &Context, pager: Pager) -> FieldResult<daffodil_log::List> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let res = daffodil_log::List::new(&context.session, db, jwt, &pager)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    async fn index_locale(context: &Context, pager: Pager) -> FieldResult<daffodil_locale::List> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let res = daffodil_locale::List::new(&context.session, db, jwt, enf, &pager).await?;
        Ok(res)
    }
    fn index_locale_by_lang(
        context: &Context,
        lang: String,
    ) -> FieldResult<Vec<daffodil_locale::Item>> {
        let db = context.postgresql.deref();
        let res = daffodil_locale::Item::by_lang(db, &lang)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    fn get_site_info_by_lang(
        context: &Context,
        lang: String,
    ) -> FieldResult<daffodil_site::info::ByLang> {
        let db = context.postgresql.deref();
        let res = daffodil_site::info::ByLang::new(db, &lang)?;
        Ok(res)
    }
    async fn get_site_author(context: &Context) -> FieldResult<petunia::themes::Author> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        let res = daffodil_site::get(&context.session, db, secrets.clone(), jwt, enf, None).await?;
        Ok(res)
    }
    async fn get_site_keywords(context: &Context) -> FieldResult<Vec<String>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        let res: daffodil_site::info::Keywords = daffodil_site::get_(
            &context.session,
            db,
            secrets.clone(),
            jwt,
            enf,
            Layout::KEYWORDS,
            None,
        )
        .await?;
        Ok(res.items)
    }
    async fn get_site_cn_icp(context: &Context) -> FieldResult<petunia::themes::CnIcp> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        let it: petunia::themes::CnIcp =
            daffodil_site::get(&context.session, db, secrets.clone(), jwt, enf, None).await?;
        Ok(it)
    }
    async fn get_site_cn_mps(context: &Context) -> FieldResult<petunia::themes::CnMps> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        let it: petunia::themes::CnMps =
            daffodil_site::get(&context.session, db, secrets.clone(), jwt, enf, None).await?;
        Ok(it)
    }
    async fn get_site_smtp(context: &Context) -> FieldResult<daffodil_site::smtp::Show> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        let it: daffodil_site::smtp::Profile =
            daffodil_site::get(&context.session, db, secrets.clone(), jwt, enf, None).await?;
        Ok(it.into())
    }
    async fn get_google_site_ownership_verifying(
        context: &Context,
    ) -> FieldResult<daffodil_site::seo::google::SiteOwnershipVerifying> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        let it: daffodil_site::seo::google::SiteOwnershipVerifying =
            daffodil_site::get(&context.session, db, secrets.clone(), jwt, enf, None).await?;
        Ok(it)
    }
    async fn get_index_now_site_ownership_verifying(
        context: &Context,
    ) -> FieldResult<daffodil_site::seo::index_now::SiteOwnershipVerifying> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        let it: daffodil_site::seo::index_now::SiteOwnershipVerifying =
            daffodil_site::get(&context.session, db, secrets.clone(), jwt, enf, None).await?;
        Ok(it)
    }
    async fn get_site_status(context: &Context) -> FieldResult<daffodil_site::status::Response> {
        let db = context.postgresql.deref();
        let ch = context.redis.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let queue = context.rabbitmq.deref();
        let minio = context.minio.deref();
        let search = context.opensearch.deref();
        let it = daffodil_site::status::Response::new(
            &context.session,
            (db, ch, queue, minio, search),
            (jwt, enf),
        )
        .await?;
        Ok(it)
    }
    // ------------------------------------------------------------------------
    async fn show_attachment(
        context: &Context,
        id: i32,
        expiration_hours: Option<i32>,
    ) -> FieldResult<daffodil_attachment::Show> {
        let db = context.postgresql.deref();
        let s3 = context.minio.deref();
        let res = daffodil_attachment::Show::new(
            db,
            s3,
            id,
            expiration_hours.map(|x| Duration::hours(x as i64)),
        )
        .await?;
        Ok(res)
    }
    fn index_attachment(context: &Context, pager: Pager) -> FieldResult<daffodil_attachment::List> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let res = daffodil_attachment::List::new(&context.session, db, jwt, &pager)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    async fn index_leave_word(
        context: &Context,
        pager: Pager,
    ) -> FieldResult<daffodil_leave_word::List> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let res = daffodil_leave_word::List::new(&context.session, db, jwt, enf, &pager).await?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    async fn index_session(context: &Context, pager: Pager) -> FieldResult<daffodil_session::List> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let res = daffodil_session::List::new(&context.session, db, jwt, enf, &pager).await?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    fn index_tag(context: &Context) -> FieldResult<Vec<daffodil_tag::Item>> {
        let db = context.postgresql.deref();
        let res = daffodil_tag::Item::all(db)?;
        Ok(res)
    }
    fn index_category(context: &Context) -> FieldResult<Vec<daffodil_category::Item>> {
        let db = context.postgresql.deref();
        let res = daffodil_category::Item::all(db)?;
        Ok(res)
    }
    fn full_tree_of_category(
        context: &Context,
        id: i32,
    ) -> FieldResult<Vec<daffodil_category::Item>> {
        let db = context.postgresql.deref();
        let res = daffodil_category::Item::retrieving_full_tree(db, id)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    fn menus(context: &Context, location: String) -> FieldResult<Vec<Menu>> {
        let db = context.postgresql.deref();
        let res = daffodil_menu::menus_by_lang_and_location(db, &context.session.lang, &location)?;
        Ok(res)
    }
    fn index(context: &Context) -> FieldResult<Vec<daffodil_menu::Item>> {
        let db = context.postgresql.deref();
        let res = daffodil_menu::Item::all(db)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    fn index_cms_page(context: &Context, pager: Pager) -> FieldResult<cms_page::List> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let res = cms_page::List::new(&context.session, db, jwt, &pager)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    fn index_questionnaire_form(
        context: &Context,
    ) -> FieldResult<Vec<wisteria_graphql::form::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let res = wisteria_graphql::form::Item::all(&context.session, db, jwt)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    fn index_questionnaire_field(
        context: &Context,
        form: i32,
    ) -> FieldResult<Vec<wisteria_graphql::field::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let res = wisteria_graphql::field::Item::by_form(&context.session, db, jwt, form)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    fn index_questionnaire_pool(
        context: &Context,
        form: i32,
    ) -> FieldResult<Vec<wisteria_graphql::poll::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let res = wisteria_graphql::poll::Item::by_form(&context.session, db, jwt, form)?;
        Ok(res)
    }
    // ------------------------------------------------------------------------
    async fn index_bookkeeping_ledger(
        context: &Context,
    ) -> FieldResult<Vec<hyacinth_graphql::ledger::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let items = hyacinth_graphql::ledger::Item::all(&context.session, db, jwt).await?;
        Ok(items)
    }
    async fn index_bookkeeping_category_by_ledger(
        context: &Context,
        id: i32,
    ) -> FieldResult<Vec<hyacinth_graphql::category::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let items =
            hyacinth_graphql::category::Item::by_ledger(&context.session, db, jwt, enf, id).await?;
        Ok(items)
    }
    async fn index_bookkeeping_account_by_ledger(
        context: &Context,
        id: i32,
    ) -> FieldResult<Vec<hyacinth_graphql::account::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let items =
            hyacinth_graphql::account::Item::by_ledger(&context.session, db, jwt, enf, id).await?;
        Ok(items)
    }
    async fn index_bookkeeping_merchant_by_ledger(
        context: &Context,
        id: i32,
    ) -> FieldResult<Vec<hyacinth_graphql::merchant::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let items =
            hyacinth_graphql::merchant::Item::by_ledger(&context.session, db, jwt, enf, id).await?;
        Ok(items)
    }
    async fn index_bookkeeping_transaction_by_ledger(
        context: &Context,
        id: i32,
    ) -> FieldResult<Vec<hyacinth_graphql::transaction::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let items =
            hyacinth_graphql::transaction::Item::by_ledger(&context.session, db, jwt, enf, id)
                .await?;
        Ok(items)
    }
    async fn index_bookkeeping_entries_by_transaction(
        context: &Context,
        id: i32,
    ) -> FieldResult<Vec<hyacinth_graphql::entry::Item>> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let items =
            hyacinth_graphql::entry::Item::by_transaction(&context.session, db, jwt, enf, id)
                .await?;
        Ok(items)
    }

    // ------------------------------------------------------------------------
    // ------------------------------------------------------------------------
}
