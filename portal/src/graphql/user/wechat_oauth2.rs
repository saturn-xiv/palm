use chrono::Duration;
use diesel::Connection as DieselConnection;
use juniper::GraphQLInputObject;
use uuid::Uuid;
use validator::Validate;

use super::super::super::{
    Error, Jwt, Plugin, Result,
    cache::redis::StandaloneConnection as Cache,
    models::{
        log::{Dao as LogDao, Level},
        user::{Dao as UserDao, Type as UserType, wechat_oauth2::Dao as WechatOauth2UserDao},
    },
    orm::postgresql::Connection as Db,
    rbac::Rbac,
    wechat::oauth2::Config as WechatOauth2,
};
use super::super::{CurrentUser, Session};
use super::{SignInResponse, State};

#[derive(Clone, Debug, Validate, GraphQLInputObject)]
#[graphql(name = "WechatOauth2UserCallbackRequest")]
pub struct Callback {
    #[validate(length(min = 1, max = 255))]
    pub code: String,
    #[validate(length(min = 2, max = 63))]
    pub state: String,
    #[validate(length(min = 2, max = 7))]
    pub lang: String,
    #[validate(length(min = 3, max = 31))]
    pub timezone: String,
}

impl Callback {
    pub async fn execute<R: Rbac, J: Jwt>(
        &self,
        ss: &Session,
        (db, cache): (&mut Db, &mut Cache),
        (rbac, jwt, wechat): (&R, &J, &WechatOauth2),
        version: &str,
    ) -> Result<SignInResponse> {
        self.validate()?;
        let state = State::new(cache, &self.state)?;

        let token = wechat.access_token(&self.code).await?;
        let info = wechat.user_info(&token.openid, &token.access_token).await?;
        log::debug!("{:?}", info);

        let ip = ss.client_ip();

        let it = db.transaction::<_, Error, _>(|tx| {
            let it = WechatOauth2UserDao::sign_in_or_up(
                tx,
                (
                    &self.lang.parse()?,
                    self.timezone.parse()?,
                    state.user.as_deref(),
                ),
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
    qr_code_url(ss, cache, wechat, ttl, None)
}

pub async fn bind_url<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    cache: &mut Cache,
    jwt: &J,
    wechat: &WechatOauth2,
    ttl: Duration,
) -> Result<String> {
    let current_user = ss.current_user(db, cache, jwt).await?;
    qr_code_url(ss, cache, wechat, ttl, Some(&current_user))
}

fn qr_code_url(
    ss: &Session,
    cache: &mut Cache,
    wechat: &WechatOauth2,
    ttl: Duration,
    user: Option<&CurrentUser>,
) -> Result<String> {
    let key = Uuid::new_v4().to_string();
    let tmp = State {
        user: user.map(|it| it.uid().clone()),
    };
    tmp.save(cache, &key, ttl)?;
    Ok(wechat.login_url(&key, &ss.locale()?))
}
