use std::any::type_name;

use chrono::Duration;
use diesel::Connection as DieselConnection;
use juniper::GraphQLInputObject;
use validator::Validate;

use super::super::super::{
    Error, Jwt, Plugin, Result,
    cache::{FlexBuffersCacher, redis::StandaloneConnection as Cache},
    models::{
        log::{Dao as LogDao, Level},
        user::{Dao as UserDao, Type as UserType, wechat_oauth2::Dao as WechatOauth2UserDao},
    },
    orm::postgresql::Connection as Db,
    random::alphanumeric as random_alphanumeric,
    rbac::Rbac,
    wechat::oauth2::Config as WechatOauth2,
};
use super::super::Session;
use super::SignInResponse;

#[derive(Clone, Debug, Validate, GraphQLInputObject)]
#[graphql(name = "UserSignInByWechatOauth2Request")]
pub struct SignIn {
    #[validate(length(min = 1, max = 255))]
    pub code: String,
    #[validate(length(min = 2, max = 63))]
    pub state: String,
    #[validate(length(min = 2, max = 7))]
    pub lang: String,
    #[validate(length(min = 3, max = 31))]
    pub timezone: String,
}

impl SignIn {
    pub async fn execute<R: Rbac, J: Jwt>(
        &self,
        ss: &Session,
        (db, cache): (&mut Db, &mut Cache),
        (rbac, jwt, wechat): (&R, &J, &WechatOauth2),
        version: &str,
    ) -> Result<SignInResponse> {
        self.validate()?;
        {
            let it = State {
                value: self.state.clone(),
            };
            it.verify(cache)?;
        }

        let token = wechat.access_token(&self.code).await?;
        let info = wechat.user_info(&token.openid, &token.access_token).await?;
        log::debug!("{:?}", info);

        let ip = ss.client_ip();

        let it = db.transaction::<_, Error, _>(|tx| {
            let it = WechatOauth2UserDao::sign_in_or_up(
                tx,
                (&self.lang.parse()?, self.timezone.parse()?),
                &wechat.app_id,
                &token,
                &info,
            )?;
            UserDao::sign_in(tx, it.user_id, ip)?;
            LogDao::create::<Plugin, _>(
                tx,
                it.user_id,
                Level::Info,
                ip,
                "Sign in by wechat oauth2.",
            )?;
            Ok(it)
        })?;

        SignInResponse::new(
            db,
            cache,
            (rbac, jwt),
            (it.user_id, UserType::WechatOauth2, &it.uid),
            version,
        )
        .await
    }
}

pub fn sign_in_url(
    ss: &Session,
    cache: &mut Cache,
    wechat: &WechatOauth2,
    ttl: Duration,
) -> Result<String> {
    let state = State::default();
    state.save(cache, ttl)?;
    Ok(wechat.login_url(&state.value, &ss.locale()?))
}

struct State {
    value: String,
}
impl Default for State {
    fn default() -> Self {
        Self {
            value: random_alphanumeric(16),
        }
    }
}
impl State {
    fn save(&self, cache: &mut Cache, ttl: Duration) -> Result<()> {
        let key = self.key();
        cache.set(&key, &true, Some(ttl))?;
        Ok(())
    }
    fn verify(&self, cache: &mut Cache) -> Result<()> {
        let key = self.key();
        let _: bool = cache.get(&key)?;
        Ok(())
    }
    fn key(&self) -> String {
        format!("{}.{}", type_name::<Self>(), self.value)
    }
}
