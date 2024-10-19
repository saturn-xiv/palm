use std::ops::Deref;

use daffodil::graphql::{
    locale as daffodil_locale, log as daffodil_log, user::email as daffodil_user_by_email,
};
use juniper::{graphql_object, FieldResult};
use petunia::{
    graphql::{Pager, Succeed},
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
}
