use std::ops::Deref;

use daffodil::graphql::user::{
    self as daffodil_user, email as daffodil_user_by_email, SignInResponse as UserSignInResponse,
};
use juniper::{graphql_object, FieldResult};
use petunia::graphql::Succeed;

use super::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn user_sign_in_by_email(
        context: &Context,
        user: String,
        password: String,
    ) -> FieldResult<UserSignInResponse> {
        let form = daffodil_user_by_email::SignIn {
            user: user.trim().to_lowercase(),
            password,
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let it = form
            .execute(db, jwt, enf, &context.session.client_ip)
            .await?;
        Ok(it)
    }
    async fn user_sign_up_by_email(
        context: &Context,
        real_name: String,
        nickname: String,
        email: String,
        password: String,
        timezone: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::SignUp {
            real_name: real_name.trim().to_string(),
            email: email.trim().to_lowercase(),
            nickname: nickname.trim().to_lowercase(),
            password,
            timezone,
            lang: context.session.lang.clone(),
        };
        let db = context.postgresql.deref();
        let queue = context.rabbitmq.deref();
        let jwt = context.jwt.deref();
        form.execute(db, jwt, queue, &context.session.client_ip)
            .await?;
        Ok(Succeed::default())
    }

    fn unlock_email_user_by_token(context: &Context, token: String) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::Token { token };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.unlock(db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    fn confirm_email_user_by_token(context: &Context, token: String) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::Token { token };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.confirm(db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    fn reset_email_user_password_by_token(
        context: &Context,
        token: String,
        password: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::ResetPassword { token, password };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    async fn confirm_email_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user_by_email::confirm(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn enable_email_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user_by_email::enable(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn disable_email_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user_by_email::disable(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }

    async fn set_email_user_password(
        context: &Context,
        id: i32,
        password: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::SetPassword { id, password };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.execute(&context.session, db, jwt, enf, &context.session.client_ip)
            .await?;
        Ok(Succeed::default())
    }

    // ------------------------------------------------------------------------

    async fn enable_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user::enable(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn disable_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user::disable(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn lock_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user::lock(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn unlock_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user::unlock(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
}
