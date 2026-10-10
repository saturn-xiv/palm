use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLInputObject;
use validator::Validate;

use super::super::super::{
    Error, HttpError, Jwt, Plugin, Result,
    cache::redis::StandaloneConnection as Cache,
    models::{
        log::{Dao as LogDao, Level},
        user::{
            Dao as UserDao, Type as UserType, wechat_mini_program::Dao as WechatMiniProgramUserDao,
        },
    },
    orm::postgresql::Connection as Db,
    rbac::Rbac,
    wechat::mini_program::Config as WechatMiniProgram,
};
use super::super::Session;
use super::SignInResponse;

#[derive(Clone, Debug, Validate, GraphQLInputObject)]
#[graphql(name = "UserSignInByWechatMiniProgramRequest")]
pub struct SignIn {
    #[validate(length(min = 1, max = 255))]
    pub code: String,
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
        (rbac, jwt, wechat): (&R, &J, &WechatMiniProgram),
        version: &str,
    ) -> Result<SignInResponse> {
        self.validate()?;
        let res = wechat.code2session(&self.code).await?;
        if res.errcode != 0 {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(res.errmsg),
            )));
        }

        let ip = ss.client_ip();

        let it = db.transaction::<_, Error, _>(|tx| {
            let it = WechatMiniProgramUserDao::sign_in_or_up(
                tx,
                (&self.lang.parse()?, self.timezone.parse()?),
                (&wechat.app_id, &res),
            )?;
            UserDao::sign_in(tx, it.user_id, ip)?;
            LogDao::create::<Plugin, _>(
                tx,
                it.user_id,
                Level::Info,
                ip,
                "Sign in by wechat mini-program.",
            )?;
            Ok(it)
        })?;

        SignInResponse::new(
            db,
            cache,
            (rbac, jwt),
            (it.user_id, UserType::WechatMiniProgram, &it.uid),
            version,
        )
        .await
    }
}

// https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/userProfile.html
#[derive(Clone, Debug, Validate)]
pub struct UserInfo {
    #[validate(length(min = 1, max = 31))]
    pub nickname: String,
    #[validate(url, length(min = 6, max = 127))]
    pub avatar_url: String,
}

impl UserInfo {
    pub async fn execute<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        jwt: &J,
    ) -> Result<()> {
        self.validate()?;

        let current_user = ss.current_user(db, cache, jwt).await?;
        if current_user.r#type != UserType::WechatMiniProgram {
            return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
        }

        let ip = ss.client_ip();

        db.transaction::<_, Error, _>(|tx| {
            let it = WechatMiniProgramUserDao::by_uid(tx, &current_user.subject)?;
            WechatMiniProgramUserDao::set_info(tx, it.id, &self.nickname, &self.avatar_url)?;

            LogDao::create::<Plugin, _>(
                tx,
                it.user_id,
                Level::Info,
                ip,
                "Update information by WeChate Mini-Program.",
            )?;
            Ok(())
        })?;

        Ok(())
    }
}
