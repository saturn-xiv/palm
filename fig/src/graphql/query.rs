use std::ops::Deref;

use chrono::Duration;
use daffodil::graphql::{
    attachment as daffodil_attachment, category as daffodil_category,
    leave_word as daffodil_leave_word, locale as daffodil_locale, log as daffodil_log,
    menu as daffodil_menu, session as daffodil_session, site as daffodil_site, tag as daffodil_tag,
    user::email as daffodil_user_by_email,
};
use juniper::{graphql_object, FieldResult};
use petunia::{
    graphql::{Pager, Succeed},
    themes::{Layout, Menu},
    GIT_VERSION,
};

use super::context::Context;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn api_version(_context: &Context) -> &str {
        GIT_VERSION
    }

    // ------------------------------------------------------------------------
    fn layout(context: &Context) -> FieldResult<Layout> {
        let db = context.postgresql.deref();
        let secrets = context.secrets.deref();
        let it = daffodil_site::layout(&context.session, db, secrets.clone())?;
        Ok(it)
    }
    // ------------------------------------------------------------------------

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
    fn index_locale_by_lang(context: &Context) -> FieldResult<Vec<daffodil_locale::Item>> {
        let db = context.postgresql.deref();
        let res = daffodil_locale::Item::by_lang(db, &context.session.lang)?;
        Ok(res)
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
    // ------------------------------------------------------------------------
    // ------------------------------------------------------------------------
}
