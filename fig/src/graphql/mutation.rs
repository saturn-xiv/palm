use std::ops::{Deref, DerefMut};

use camelia::graphql as camelia_graphql;
use daffodil::graphql as daffodil_graphql;
use juniper::{graphql_object, FieldResult};
use palm::{Succeed, TextEditor};

use super::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn sign_in_user_by_email(
        context: &Context,
        user: String,
        password: String,
        ttl: Option<i32>,
    ) -> FieldResult<camelia_graphql::user::SignInResponse> {
        let request = camelia_graphql::user::SignInRequest {
            user: user.trim().to_string(),
            password,
            ttl: ttl.unwrap_or(60 * 60 * 24) as i64,
        };
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let mac = context.hmac.deref();
        let enf = context.enforcer.deref();
        let it = request.handle(&context.session, db, enf, jwt, mac).await?;
        Ok(it)
    }
    async fn sign_up_user_by_email(
        context: &Context,
        real_name: String,
        nickname: String,
        email: String,
        password: String,
        home: String,
        timezone: String,
    ) -> FieldResult<Succeed> {
        let request = camelia_graphql::user::SignUpRequest {
            real_name: real_name.trim().to_string(),
            nickname: nickname.trim().to_string(),
            email: email.trim().to_string(),
            password,
            timezone,
            home,
        };
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let mac = context.hmac.deref();
        let rabbitmq = context.rabbitmq.deref();

        request
            .handle(&context.session, db, jwt, mac, rabbitmq)
            .await?;
        Ok(Succeed::default())
    }
    async fn unlock_user_by_email(
        context: &Context,
        user: String,
        home: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let rabbitmq = context.rabbitmq.deref();
        let request = camelia_graphql::user::ByEmail { user, home };
        request.unlock(db, jwt, rabbitmq).await?;
        Ok(Succeed::default())
    }
    fn unlock_user_by_token(context: &Context, token: String) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let request = camelia_graphql::user::ByToken { token };
        request.unlock(&context.session, db, jwt)?;
        Ok(Succeed::default())
    }
    async fn confirm_user_by_email(
        context: &Context,
        user: String,
        home: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let rabbitmq = context.rabbitmq.deref();
        let request = camelia_graphql::user::ByEmail { user, home };
        request.confirm(db, jwt, rabbitmq).await?;
        Ok(Succeed::default())
    }
    fn confirm_user_by_token(context: &Context, token: String) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let request = camelia_graphql::user::ByToken { token };
        request.confirm(&context.session, db, jwt)?;
        Ok(Succeed::default())
    }
    async fn forgot_user_password(
        context: &Context,
        user: String,
        home: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let rabbitmq = context.rabbitmq.deref();
        let request = camelia_graphql::user::ByEmail { user, home };
        request.forgot_password(db, jwt, rabbitmq).await?;
        Ok(Succeed::default())
    }
    fn reset_user_password(
        context: &Context,
        token: String,
        password: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let mac = context.hmac.deref();
        let request = camelia_graphql::user::ResetPassword { token, password };
        request.handle(&context.session, db, jwt, mac)?;
        Ok(Succeed::default())
    }
    async fn refresh_user_token(context: &Context, ttl: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let request = camelia_graphql::user::RefreshToken { ttl: ttl as i64 };
        request.handle(&context.session, db, ch, enf, jwt).await?;
        Ok(Succeed::default())
    }

    fn sign_out_user(context: &Context) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        camelia_graphql::user::sign_out(&context.session, db, ch, jwt)?;
        Ok(Succeed::default())
    }

    fn update_user_profile(
        context: &Context,
        real_name: String,
        avatar: String,
        lang: String,
        timezone: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let request = camelia_graphql::user::UpdateProfile {
            real_name: real_name.trim().to_string(),
            avatar,
            lang: lang.trim().to_string(),
            timezone,
        };
        request.handle(&context.session, db, ch, jwt)?;
        Ok(Succeed::default())
    }
    fn change_user_password(
        context: &Context,
        current_password: String,
        new_password: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let mac = context.hmac.deref();

        let request = camelia_graphql::user::ChangePassword {
            current_password,
            new_password,
        };
        request.handle(&context.session, db, ch, jwt, mac)?;
        Ok(Succeed::default())
    }

    async fn enable_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        camelia_graphql::user::enable(&context.session, db, ch, enf, jwt, id).await?;

        Ok(Succeed::default())
    }
    async fn disable_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        camelia_graphql::user::disable(&context.session, db, ch, enf, jwt, id).await?;

        Ok(Succeed::default())
    }

    async fn lock_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        camelia_graphql::user::lock(&context.session, db, ch, enf, jwt, id).await?;

        Ok(Succeed::default())
    }

    async fn unlock_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        camelia_graphql::user::unlock(&context.session, db, ch, enf, jwt, id).await?;

        Ok(Succeed::default())
    }

    async fn confirm_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        camelia_graphql::user::confirm(&context.session, db, ch, enf, jwt, id).await?;

        Ok(Succeed::default())
    }

    async fn set_user_password(
        context: &Context,
        user: i32,
        password: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let mac = context.hmac.deref();
        let enf = context.enforcer.deref();
        let request = camelia_graphql::user::SetPassword { user, password };

        request
            .handle(&context.session, db, ch, enf, jwt, mac)
            .await?;

        Ok(Succeed::default())
    }

    async fn set_locale(
        context: &Context,
        lang: String,
        code: String,
        message: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let request = camelia_graphql::locale::Form {
            lang: lang.trim().to_string(),
            code: code.trim().to_string(),
            message: message.trim().to_string(),
        };

        request.handle(&context.session, db, ch, enf, jwt).await?;
        Ok(Succeed::default())
    }

    async fn set_baidu_site_verification(
        context: &Context,
        content_code: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let aes = context.aes.deref();
        let enf = context.enforcer.deref();
        let request = camelia_graphql::site::seo::BaiduSiteVerification { content_code };

        request.set(&context.session, db, ch, enf, jwt, aes).await?;
        Ok(Succeed::default())
    }

    async fn destroy_attachment(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        camelia_graphql::attachment::destroy(&context.session, db, ch, enf, jwt, id).await?;

        Ok(Succeed::default())
    }
    async fn create_leave_word(
        context: &Context,
        content: String,
        editor: TextEditor,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();

        let request = camelia_graphql::leave_words::Form { content, editor };

        request.handle(&context.session, db).await?;

        Ok(Succeed::default())
    }
    async fn destroy_leave_word(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        camelia_graphql::leave_words::destroy(&context.session, db, ch, enf, jwt, id).await?;

        Ok(Succeed::default())
    }

    fn daffodil_create_ledger(
        context: &Context,
        name: String,
        summary: String,
        cover: i32,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();

        let request = daffodil_graphql::ledger::Form {
            name: name.trim().to_string(),
            summary,
            cover,
        };

        request.create(&context.session, db, ch, jwt)?;

        Ok(Succeed::default())
    }
    async fn daffodil_update_ledger(
        context: &Context,
        id: i32,
        name: String,
        summary: String,
        cover: i32,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let request = daffodil_graphql::ledger::Form {
            name: name.trim().to_string(),
            summary,
            cover,
        };

        request
            .update(&context.session, db, ch, enf, jwt, id)
            .await?;

        Ok(Succeed::default())
    }
    async fn daffodil_enable_ledger(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        daffodil_graphql::ledger::enable(&context.session, db, ch, enf, jwt, id, true).await?;

        Ok(Succeed::default())
    }
    async fn daffodil_disable_ledger(context: &Context, id: i32) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        daffodil_graphql::ledger::enable(&context.session, db, ch, enf, jwt, id, false).await?;

        Ok(Succeed::default())
    }
    async fn daffodil_create_bill(
        context: &Context,
        ledger: i32,
        form: daffodil_graphql::bill::Form,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        form.create(&context.session, db, ch, enf, jwt, ledger)
            .await?;

        Ok(Succeed::default())
    }
    async fn daffodil_update_bill(
        context: &Context,
        id: i32,
        form: daffodil_graphql::bill::Form,
        reason: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        form.update(&context.session, db, ch, enf, jwt, (id, &reason))
            .await?;

        Ok(Succeed::default())
    }
    async fn daffodil_destroy_bill(
        context: &Context,
        id: i32,
        reason: String,
    ) -> FieldResult<Succeed> {
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let mut ch = context.redis.get()?;
        let ch = ch.deref_mut();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        daffodil_graphql::bill::delete(&context.session, db, ch, enf, jwt, (id, &reason)).await?;

        Ok(Succeed::default())
    }
}
