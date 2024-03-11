use std::ops::{Deref, DerefMut};

use camelia::graphql as camelia_graphql;
use juniper::{graphql_object, FieldResult};
use palm::Succeed;

use super::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn signIn(
        context: &Context,
        user: String,
        password: String,
        ttl: Option<i32>,
    ) -> FieldResult<camelia_graphql::user::SignInResponse> {
        let request = camelia_graphql::user::SignInRequest {
            user,
            password,
            ttl: ttl.unwrap_or(60 * 60 * 24) as i64,
        };
        let mut db = context.postgresql.get()?;
        let db = db.deref_mut();
        let jwt = context.jwt.deref();
        let mac = context.hmac.deref();
        let enforcer = context.enforcer.deref();
        let it = request
            .handle(&context.session, db, jwt, mac, enforcer)
            .await?;
        Ok(it)
    }
    async fn signUp(
        context: &Context,
        real_name: String,
        nickname: String,
        email: String,
        password: String,
        home: String,
        lang: String,
        timezone: String,
    ) -> FieldResult<Succeed> {
        let request = camelia_graphql::user::SignUpRequest {
            real_name,
            nickname,
            email,
            password,
            lang,
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
}
