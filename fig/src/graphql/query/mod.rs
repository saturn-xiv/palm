pub mod menu;

use std::any::type_name;
use std::ops::{Deref, DerefMut};

use camelia::graphql as camelia_graphql;
use daffodil::graphql as daffodil_graphql;
use juniper::{graphql_object, FieldResult};
use palm::{iso4217, pagination::Pager, Succeed, GIT_VERSION};

use super::context::Context;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn api_version(_context: &Context) -> &str {
        GIT_VERSION
    }
    fn resource_types(_context: &Context) -> Vec<&str> {
        vec![
            type_name::<daffodil::models::ledger::Item>(),
            type_name::<daffodil::models::bill::Item>(),
        ]
    }
    fn currencies(_context: &Context) -> FieldResult<Vec<iso4217::list_one::Item>> {
        let items = iso4217::list_one::Item::all()?;
        Ok(items)
    }
    fn currency_options(_context: &Context) -> FieldResult<Vec<iso4217::Currency>> {
        let items = iso4217::Currency::all()?;
        Ok(items)
    }
    fn site_info(
        context: &Context,
        lang: String,
    ) -> FieldResult<camelia_graphql::site::info::Response> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let aes = context.aes.deref();
        let response = camelia_graphql::site::info::Response::new(db, ch, aes, &lang)?;
        Ok(response)
    }
    async fn routes(context: &Context) -> FieldResult<Vec<menu::Route>> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let response = menu::Route::load(&context.session, db, ch, enf, jwt).await?;
        Ok(response)
    }

    async fn get_baidu_site_verification(
        context: &Context,
    ) -> FieldResult<palm::seo::baidu::SiteVerification> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let aes = context.aes.deref();
        let enf = context.enforcer.deref();

        let it = camelia_graphql::site::get(&context.session, db, ch, enf, jwt, aes)
            .await
            .unwrap_or_default();
        Ok(it)
    }
    async fn ping_baidu(context: &Context, home: String) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let request = camelia_graphql::site::seo::Ping { home };
        request.baidu(&context.session, db, ch, enf, jwt).await?;
        Ok(Succeed::default())
    }
    async fn get_google_site_verification(
        context: &Context,
    ) -> FieldResult<palm::seo::google::SiteVerification> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let aes = context.aes.deref();
        let enf = context.enforcer.deref();

        let it = camelia_graphql::site::get(&context.session, db, ch, enf, jwt, aes)
            .await
            .unwrap_or_default();
        Ok(it)
    }
    async fn get_google_recaptcha(context: &Context) -> FieldResult<palm::seo::google::ReCaptcha> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let aes = context.aes.deref();
        let enf = context.enforcer.deref();

        let it = camelia_graphql::site::get(&context.session, db, ch, enf, jwt, aes)
            .await
            .unwrap_or_default();
        Ok(it)
    }
    async fn ping_google(context: &Context, home: String) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let request = camelia_graphql::site::seo::Ping { home };
        request.google(&context.session, db, ch, enf, jwt).await?;
        Ok(Succeed::default())
    }
    async fn get_index_now_site_verification(
        context: &Context,
    ) -> FieldResult<palm::seo::index_now::SiteVerification> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let aes = context.aes.deref();
        let enf = context.enforcer.deref();

        let it = camelia_graphql::site::get(&context.session, db, ch, enf, jwt, aes)
            .await
            .unwrap_or_default();
        Ok(it)
    }
    async fn ping_index_now(context: &Context, home: String) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let aes = context.aes.deref();
        let enf = context.enforcer.deref();

        let request = camelia_graphql::site::seo::Ping { home };
        request
            .index_now(&context.session, db, ch, enf, jwt, aes)
            .await?;
        Ok(Succeed::default())
    }

    fn index_locale(
        context: &Context,
        pager: Pager,
    ) -> FieldResult<camelia_graphql::locale::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let response = camelia_graphql::locale::IndexResponse::new(db, &pager)?;
        Ok(response)
    }
    fn index_locale_by_lang(
        context: &Context,
        lang: String,
    ) -> FieldResult<Vec<camelia_graphql::locale::IndexResponseItem>> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let response = camelia_graphql::locale::IndexResponseItem::by_lang(db, &lang)?;
        Ok(response)
    }
    async fn current_user(context: &Context) -> FieldResult<camelia_graphql::user::CurrentUser> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response =
            camelia_graphql::user::CurrentUser::refresh(&context.session, db, ch, enf, jwt).await?;
        Ok(response)
    }
    fn logs(context: &Context, pager: Pager) -> FieldResult<camelia_graphql::log::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let response =
            camelia_graphql::log::IndexResponse::new(&context.session, db, ch, jwt, &pager)?;
        Ok(response)
    }
    async fn index_user(
        context: &Context,
        pager: Pager,
    ) -> FieldResult<camelia_graphql::user::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response =
            camelia_graphql::user::IndexResponse::new(&context.session, db, ch, enf, jwt, &pager)
                .await?;

        Ok(response)
    }
    async fn show_user(
        context: &Context,
        id: i32,
    ) -> FieldResult<camelia_graphql::user::IndexResponseItem> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response =
            camelia_graphql::user::IndexResponseItem::new(&context.session, db, ch, enf, jwt, id)
                .await?;

        Ok(response)
    }
    fn index_picture(
        context: &Context,
    ) -> FieldResult<Vec<camelia_graphql::attachment::IndexResponseItem>> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let response = camelia_graphql::attachment::IndexResponseItem::pictures(
            &context.session,
            db,
            ch,
            jwt,
        )?;

        Ok(response)
    }
    fn index_attachment(
        context: &Context,
        pager: Pager,
    ) -> FieldResult<camelia_graphql::attachment::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let response =
            camelia_graphql::attachment::IndexResponse::new(&context.session, db, ch, jwt, &pager)?;

        Ok(response)
    }
    async fn index_attachment_by_resource(
        context: &Context,
        operation: String,
        resource_type: String,
        resource_id: i32,
    ) -> FieldResult<Vec<camelia_graphql::attachment::IndexResponseItem>> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response = camelia_graphql::attachment::IndexResponseItem::by_resource(
            &context.session,
            db,
            ch,
            enf,
            jwt,
            &operation,
            (&resource_type, resource_id),
        )
        .await?;

        Ok(response)
    }
    async fn show_attachment_by_id(
        context: &Context,
        id: i32,
        ttl: Option<i32>,
    ) -> FieldResult<camelia_graphql::attachment::ShowResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let s3 = context.minio.deref();

        let request = camelia_graphql::attachment::ByIdRequest {
            id,
            ttl: ttl.map(|x| x as i64),
        };
        let response = request
            .handle(&context.session, db, ch, enf, jwt, s3)
            .await?;

        Ok(response)
    }
    async fn show_attachment_by_bucket_and_name(
        context: &Context,
        bucket: String,
        name: String,
        ttl: Option<i32>,
    ) -> FieldResult<camelia_graphql::attachment::ShowResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let s3 = context.minio.deref();

        let request = camelia_graphql::attachment::ByBucketAndNameRequest {
            bucket,
            name,
            ttl: ttl.map(|x| x as i64),
        };
        let response = request
            .handle(&context.session, db, ch, enf, jwt, s3)
            .await?;

        Ok(response)
    }
    async fn index_leave_word(
        context: &Context,
        pager: Pager,
    ) -> FieldResult<camelia_graphql::leave_words::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response = camelia_graphql::leave_words::IndexResponse::new(
            &context.session,
            db,
            ch,
            enf,
            jwt,
            &pager,
        )
        .await?;

        Ok(response)
    }
    async fn daffodil_show_ledger(
        context: &Context,
        id: i32,
    ) -> FieldResult<daffodil_graphql::ledger::IndexResponseItem> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let s3 = context.minio.deref();
        let enf = context.enforcer.deref();

        let response = daffodil_graphql::ledger::IndexResponseItem::show(
            &context.session,
            db,
            ch,
            enf,
            jwt,
            s3,
            id,
        )
        .await?;

        Ok(response)
    }
    async fn daffodil_share_ledger(
        context: &Context,
        id: i32,
        begin: String,
        end: String,
    ) -> FieldResult<String> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response =
            daffodil_graphql::ledger::share(&context.session, db, ch, enf, jwt, id, (&begin, &end))
                .await?;

        Ok(response)
    }
    async fn daffodil_index_ledger(
        context: &Context,
    ) -> FieldResult<daffodil_graphql::ledger::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let s3 = context.minio.deref();

        let response =
            daffodil_graphql::ledger::IndexResponse::new(&context.session, db, ch, jwt, s3).await?;

        Ok(response)
    }
    async fn daffodil_export_ledger(
        context: &Context,
        id: i32,
        format: daffodil_graphql::ledger::ExportFormat,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let request = daffodil_graphql::ledger::ExportRequest { id, format };
        request.handle(&context.session, db, ch, enf, jwt).await?;

        Ok(Succeed::default())
    }
    async fn daffodil_show_bill(
        context: &Context,
        id: i32,
    ) -> FieldResult<daffodil_graphql::bill::IndexResponseItem> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response =
            daffodil_graphql::bill::IndexResponseItem::show(&context.session, db, ch, enf, jwt, id)
                .await?;

        Ok(response)
    }
    async fn daffodil_index_bill(
        context: &Context,
        ledger: i32,
    ) -> FieldResult<daffodil_graphql::bill::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response =
            daffodil_graphql::bill::IndexResponse::new(&context.session, db, ch, enf, jwt, ledger)
                .await?;

        Ok(response)
    }
    async fn daffodil_index_bill_history(
        context: &Context,
        ledger: i32,
    ) -> FieldResult<daffodil_graphql::bill::history::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let response = daffodil_graphql::bill::history::IndexResponse::new(
            &context.session,
            db,
            ch,
            enf,
            jwt,
            ledger,
        )
        .await?;

        Ok(response)
    }
    fn daffodil_bill_merchants(context: &Context) -> FieldResult<Vec<String>> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let response = daffodil_graphql::bill::merchants(&context.session, db, ch, jwt)?;

        Ok(response)
    }
    fn daffodil_bill_categories(context: &Context) -> FieldResult<Vec<String>> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let response = daffodil_graphql::bill::categories(&context.session, db, ch, jwt)?;

        Ok(response)
    }
    fn daffodil_bill_payment_methods(context: &Context) -> FieldResult<Vec<String>> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let response = daffodil_graphql::bill::payment_methods(&context.session, db, ch, jwt)?;

        Ok(response)
    }
}
