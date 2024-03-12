use std::ops::{Deref, DerefMut};

use camelia::graphql as camelia_graphql;
use daffodil::graphql as daffodil_graphql;
use juniper::{graphql_object, FieldResult};
use palm::{pagination::Pager, GIT_VERSION};

use super::context::Context;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn apiVersion(_context: &Context) -> &str {
        GIT_VERSION
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

    fn daffodil_index_ledger(
        context: &Context,
    ) -> FieldResult<daffodil_graphql::ledger::IndexResponse> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let response = daffodil_graphql::ledger::IndexResponse::new(&context.session, db, ch, jwt)?;

        Ok(response)
    }
    async fn daffodil_export_ledger(
        context: &Context,
        id: i32,
        r#type: daffodil_graphql::ledger::ExportType,
        home: String,
        ttl: i32,
    ) -> FieldResult<String> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let request = daffodil_graphql::ledger::ExportRequest {
            id,
            ttl: ttl as i64,
            home,
            r#type,
        };
        let response = request.handle(&context.session, db, ch, enf, jwt).await?;

        Ok(response)
    }
}
