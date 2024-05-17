use std::collections::{BTreeSet, HashMap};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::sync::Arc;

use chrono::Duration;
use chrono_tz::Tz;
use diesel::Connection as DieselConntection;
use hibiscus::{
    cache::redis::Pool as CachePool,
    queue::rabbitmq::{
        amqp::{Amqp, Thrift as ThriftAmqpProtocol},
        Config as RabbitMq,
    },
};
use hyper::StatusCode;
use language_tags::LanguageTag;
use palm::to_chrono_duration;
use palm::{
    azalea::v1::{IdRequest, Pager, Pagination},
    camelia::v1,
    daisy::protocols::{Address as EmailAddress, Body as EmailBody, EmailSendTask},
    gourd::Policy,
    random::uuid,
    to_code, to_timestamp, try_grpc,
    tuberose::protocols::SmsSendTask,
    Error, GrpcResult, HttpError, Jwt, Password, Result, Thrift,
};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::super::{
    i18n::I18n,
    models::{
        google::user::{Dao as GoogleUserDao, Item as GoogleUser},
        log::{Dao as LogDao, Item as Log},
        user::{
            email::{Dao as EmailUserDao, Item as EmailUser},
            session::Dao as SessionDao,
            {Dao as UserDao, Item as User},
        },
        wechat::{
            mini_program_user::{Dao as WechatMiniProgramDao, Item as WechatMiniProgramUser},
            oauth2_user::{Dao as WechatOauth2Dao, Item as WechatOauth2User},
        },
    },
    orm::postgresql::{Connection as Db, Pool as DbPool},
    NAME,
};
use super::{CurrentUserAdapter, Session};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
    pub queue: Arc<RabbitMq>,
}

#[tonic::async_trait]
impl v1::user_server::User for Service {
    async fn sign_in_by_password(
        &self,
        req: Request<v1::UserSignInByPasswordRequest>,
    ) -> GrpcResult<v1::UserSignInResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt_hmac = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        let (user, detail, provider_id) =
            try_grpc!(user_from_sign_in_by_password_request(db, jwt_hmac, &req))?;
        let res = try_grpc!(new_sign_in_response(
            db,
            jwt_hmac,
            policy,
            (&user, detail, provider_id),
            (
                &ss.client_ip,
                req.ttl
                    .map(|x| to_chrono_duration!(x))
                    .unwrap_or(Duration::days(1))
            ),
        ))?;
        Ok(Response::new(res))
    }
    async fn sign_up_by_email(&self, req: Request<v1::UserSignUpByEmailRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt_hmac = self.loquat.deref();
        let queue = self.queue.deref();
        let req = req.into_inner();

        let email = to_code!(req.email);
        let nickname = to_code!(req.nickname);
        {
            let form = SignUpByEmailForm {
                nickname: &nickname,
                email: &email,
                real_name: &req.real_name,
                password: &req.password,
                timezone: &req.timezone,
                home: &req.home,
            };
            try_grpc!(form.validate())?
        }
        let timezone = try_grpc!(req.timezone.parse::<Tz>())?;
        let lang = try_grpc!(LanguageTag::from_str(&ss.lang))?;
        let real_name = req.real_name.clone();

        {
            let email = email.clone();
            let real_name = real_name.clone();
            try_grpc!(db.transaction::<_, Error, _>(move |db| {
                let user = EmailUserDao::create(
                    db,
                    jwt_hmac,
                    &real_name,
                    &nickname,
                    &email,
                    &req.password,
                    (&lang, timezone),
                )?;

                LogDao::add::<_, User>(
                    db,
                    user.id,
                    NAME,
                    v1::user_logs_response::item::Level::Info,
                    &ss.client_ip,
                    None,
                    "create account by email",
                )?;

                Ok(())
            }))?;
        }

        try_grpc!(
            send_email(
                db,
                jwt_hmac,
                queue,
                &ss.lang,
                (&real_name, &email),
                &req.home,
                v1::UserTokenAction::Confirm
            )
            .await
        )?;
        Ok(Response::new(()))
    }
    async fn confirm_by_email(&self, req: Request<v1::UserEmailRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt = self.loquat.deref();
        let queue = self.queue.deref();
        let req = req.into_inner();

        {
            let form = HomeForm { home: &req.home };
            try_grpc!(form.validate())?
        }

        let it = try_grpc!(email_user_by_from_request(db, &req))?;
        {
            if it.deleted_at.is_some() {
                return Err(Status::invalid_argument("user is deleted"));
            }
            if it.confirmed_at.is_some() {
                return Err(Status::invalid_argument("user is confirmed"));
            }
        }
        {
            let user = try_grpc!(UserDao::by_id(db, it.user_id))?;
            if user.deleted_at.is_some() {
                return Err(Status::invalid_argument("user is deleted"));
            }
        }
        try_grpc!(
            send_email(
                db,
                jwt,
                queue,
                &ss.lang,
                (&it.real_name, &it.email),
                &req.home,
                v1::UserTokenAction::Confirm,
            )
            .await
        )?;

        Ok(Response::new(()))
    }
    async fn confirm_by_token(&self, req: Request<v1::UserTokenRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        let (_, sid, provider_type) = try_grpc!(Jwt::verify::<i32>(
            jwt,
            &req.token,
            NAME,
            v1::UserTokenAction::Confirm.as_str_name()
        ))?;

        if let Some(provider_type) = provider_type {
            let provider_type =
                try_grpc!(v1::user_index_response::item::Type::try_from(provider_type))?;
            if provider_type == v1::user_index_response::item::Type::Email {
                try_grpc!(db.transaction::<_, Error, _>(move |db| {
                    let it = EmailUserDao::by_email(db, &sid)?;
                    EmailUserDao::confirm(db, it.id)?;
                    LogDao::add::<_, User>(
                        db,
                        it.user_id,
                        NAME,
                        v1::user_logs_response::item::Level::Info,
                        &ss.client_ip,
                        None,
                        "confirm email account",
                    )?;

                    Ok(())
                }))?;
                return Ok(Response::new(()));
            }
        }

        Err(Status::invalid_argument("not supported yet"))
    }
    async fn unlock_by_email(&self, req: Request<v1::UserEmailRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt = self.loquat.deref();
        let queue = self.queue.deref();
        let req = req.into_inner();

        {
            let form = HomeForm { home: &req.home };
            try_grpc!(form.validate())?
        }

        let it = try_grpc!(email_user_by_from_request(db, &req))?;
        try_grpc!(it.available())?;
        {
            let user = try_grpc!(UserDao::by_id(db, it.user_id))?;
            if user.deleted_at.is_some() {
                return Err(Status::invalid_argument("user is deleted"));
            }
            if user.locked_at.is_none() {
                return Err(Status::invalid_argument("user isn't locked"));
            }
        }
        try_grpc!(
            send_email(
                db,
                jwt,
                queue,
                &ss.lang,
                (&it.real_name, &it.email),
                &req.home,
                v1::UserTokenAction::Unlock,
            )
            .await
        )?;

        Ok(Response::new(()))
    }
    async fn unlock_by_token(&self, req: Request<v1::UserTokenRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        let (_, sid, provider_type) = try_grpc!(Jwt::verify::<i32>(
            jwt,
            &req.token,
            NAME,
            v1::UserTokenAction::Unlock.as_str_name()
        ))?;

        if let Some(provider_type) = provider_type {
            let provider_type =
                try_grpc!(v1::user_index_response::item::Type::try_from(provider_type))?;
            if provider_type == v1::user_index_response::item::Type::Email {
                try_grpc!(db.transaction::<_, Error, _>(move |db| {
                    let it = EmailUserDao::by_email(db, &sid)?;
                    UserDao::lock(db, it.user_id, false)?;
                    LogDao::add::<_, User>(
                        db,
                        it.user_id,
                        NAME,
                        v1::user_logs_response::item::Level::Info,
                        &ss.client_ip,
                        None,
                        "unlock account",
                    )?;

                    Ok(())
                }))?;
                return Ok(Response::new(()));
            }
        }

        Err(Status::invalid_argument("not supported yet"))
    }
    async fn forgot_password(&self, req: Request<v1::UserEmailRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt = self.loquat.deref();
        let queue = self.queue.deref();
        let req = req.into_inner();

        {
            let form = HomeForm { home: &req.home };
            try_grpc!(form.validate())?
        }

        let it = try_grpc!(email_user_by_from_request(db, &req))?;
        try_grpc!(it.available())?;
        {
            let user = try_grpc!(UserDao::by_id(db, it.user_id))?;
            try_grpc!(user.available())?;
        }
        try_grpc!(
            send_email(
                db,
                jwt,
                queue,
                &ss.lang,
                (&it.real_name, &it.email),
                &req.home,
                v1::UserTokenAction::ResetPassword,
            )
            .await
        )?;

        Ok(Response::new(()))
    }

    async fn reset_password(&self, req: Request<v1::UserResetPasswordRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt_hmac = self.loquat.deref();
        let req = req.into_inner();

        {
            let form = ResetPasswordForm {
                password: &req.password,
                token: &req.token,
            };
            try_grpc!(form.validate())?;
        }
        let (_, sid, provider_type) = try_grpc!(Jwt::verify::<i32>(
            jwt_hmac,
            &req.token,
            NAME,
            v1::UserTokenAction::ResetPassword.as_str_name()
        ))?;

        if let Some(provider_type) = provider_type {
            let provider_type =
                try_grpc!(v1::user_index_response::item::Type::try_from(provider_type))?;
            if provider_type == v1::user_index_response::item::Type::Email {
                try_grpc!(db.transaction::<_, Error, _>(move |db| {
                    let it = EmailUserDao::by_email(db, &sid)?;
                    it.available()?;
                    EmailUserDao::password(db, jwt_hmac, it.id, &req.password)?;
                    LogDao::add::<_, User>(
                        db,
                        it.user_id,
                        NAME,
                        v1::user_logs_response::item::Level::Info,
                        &ss.client_ip,
                        None,
                        "reset password",
                    )?;

                    Ok(())
                }))?;
                return Ok(Response::new(()));
            }
        }

        Err(Status::invalid_argument("not supported yet"))
    }

    async fn logs(&self, req: Request<Pager>) -> GrpcResult<v1::UserLogsResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let pager = req.into_inner();

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        let total = try_grpc!(LogDao::count(db, user.id))?;
        let items = try_grpc!(LogDao::all(db, user.id, pager.offset(total), pager.size()))?;
        Ok(Response::new(v1::UserLogsResponse {
            items: items.into_iter().map(|x| x.into()).collect(),
            pagination: Some(Pagination::new(&pager, total)),
        }))
    }
    async fn change_password(&self, req: Request<v1::UserChangePasswordRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_hmac = self.loquat.deref();
        let req = req.into_inner();

        let (_, _, (provider_type, provider_id)) = try_grpc!(ss.current_user(db, ch, jwt_hmac))?;

        {
            let form = ChangePasswordForm {
                current_password: &req.current_password,
                new_password: &req.new_password,
            };
            try_grpc!(form.validate())?;
        }
        if provider_type != v1::user_index_response::item::Type::Email {
            try_grpc!(db.transaction::<_, Error, _>(move |db| {
                let it = EmailUserDao::by_id(db, provider_id)?;
                it.auth(jwt_hmac, &req.current_password)?;
                EmailUserDao::password(db, jwt_hmac, it.id, &req.new_password)?;
                LogDao::add::<_, User>(
                    db,
                    it.user_id,
                    NAME,
                    v1::user_logs_response::item::Level::Info,
                    &ss.client_ip,
                    None,
                    "change password",
                )?;

                Ok(())
            }))?;

            return Ok(Response::new(()));
        }
        Err(Status::invalid_argument(format!(
            "not supported for {} account yet",
            provider_type.as_str_name()
        )))
    }
    async fn set_location(&self, req: Request<v1::UserSetLocationRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;

        {
            let form = LocationForm {
                lang: &req.lang,
                timezone: &req.timezone,
            };
            try_grpc!(form.validate())?;
        }
        let lang = try_grpc!(LanguageTag::from_str(&req.lang))?;
        let timezone = try_grpc!(req.timezone.parse::<Tz>())?;
        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            UserDao::set_lang(db, user.id, &lang)?;
            UserDao::set_timezone(db, user.id, timezone)?;
            LogDao::add::<_, User>(
                db,
                user.id,
                NAME,
                v1::user_logs_response::item::Level::Info,
                &ss.client_ip,
                None,
                "update location",
            )?;

            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn set_profile(&self, req: Request<v1::UserSetProfileRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        let (_, _, (provider_type, provider_id)) = try_grpc!(ss.current_user(db, ch, jwt))?;

        if provider_type != v1::user_index_response::item::Type::Email {
            {
                let form = ProfileForm {
                    real_name: &req.real_name,
                    avatar: &req.avatar,
                };
                try_grpc!(form.validate())?;
            }
            try_grpc!(db.transaction::<_, Error, _>(move |db| {
                let it = EmailUserDao::by_id(db, provider_id)?;
                EmailUserDao::set_avatar(db, it.id, &req.avatar)?;
                EmailUserDao::set_real_name(db, it.id, &req.real_name)?;
                LogDao::add::<_, User>(
                    db,
                    it.user_id,
                    NAME,
                    v1::user_logs_response::item::Level::Info,
                    &ss.client_ip,
                    None,
                    "update profile",
                )?;

                Ok(())
            }))?;

            return Ok(Response::new(()));
        }
        Err(Status::invalid_argument(format!(
            "not supported for {} account yet",
            provider_type.as_str_name()
        )))
    }

    async fn cancel(
        &self,
        req: Request<v1::UserCancelRequest>,
    ) -> GrpcResult<v1::UserCancelResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();
        let queue = self.queue.deref();

        {
            let form = HomeForm { home: &req.home };
            try_grpc!(form.validate())?;
        }

        let (user, _, (provider_type, provider_id)) = try_grpc!(ss.current_user(db, ch, jwt))?;
        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            SessionDao::delete_by_user(db, user.id)?;
            LogDao::add::<_, User>(
                db,
                user.id,
                NAME,
                v1::user_logs_response::item::Level::Warn,
                &ss.client_ip,
                None,
                format!(
                    "try to cancel by self({}), reason: {}",
                    provider_type.as_str_name(),
                    req.reason
                ),
            )?;

            Ok(())
        }))?;

        match provider_type {
            v1::user_index_response::item::Type::Email => {
                let it = try_grpc!(EmailUserDao::by_id(db, provider_id))?;
                try_grpc!(it.available())?;
                try_grpc!(
                    send_email(
                        db,
                        jwt,
                        queue,
                        &ss.lang,
                        (&it.real_name, &it.email),
                        &req.home,
                        v1::UserTokenAction::Cancel
                    )
                    .await
                )?;
                Ok(Response::new(v1::UserCancelResponse {
                    payload: Some(v1::user_cancel_response::Payload::Reminder(
                        "you will receive an email to delete your account.".to_string(),
                    )),
                }))
            }
            v1::user_index_response::item::Type::Google => {
                // TODO
                let token = "xxx";
                Ok(Response::new(v1::UserCancelResponse {
                    payload: Some(v1::user_cancel_response::Payload::RedirectTo(format!(
                        "{}/google/oauth2/cancel-callback/{}",
                        req.home, token
                    ))),
                }))
            }
            v1::user_index_response::item::Type::WechatOauth2 => {
                // TODO
                let token = "xxx";
                Ok(Response::new(v1::UserCancelResponse {
                    payload: Some(v1::user_cancel_response::Payload::RedirectTo(format!(
                        "{}/wechat/oauth2/cancel-callback/{}",
                        req.home, token
                    ))),
                }))
            }
            v1::user_index_response::item::Type::WechatMiniProgram => {
                // TODO
                let token = "xxx";
                Ok(Response::new(v1::UserCancelResponse {
                    payload: Some(v1::user_cancel_response::Payload::RedirectTo(format!(
                        "{}/wechat/mini-program/cancel-callback/{}",
                        req.home, token
                    ))),
                }))
            }
            v1::user_index_response::item::Type::Phone => {
                // TODO
                let name = "who-am-i";
                let phone = "xxx";
                try_grpc!(
                    send_sms(
                        db,
                        jwt,
                        queue,
                        &ss.lang,
                        (name, phone),
                        &req.home,
                        v1::UserTokenAction::Cancel
                    )
                    .await
                )?;
                Ok(Response::new(v1::UserCancelResponse {
                    payload: Some(v1::user_cancel_response::Payload::Reminder(
                        "you will receive an sms to cancel your account.".to_string(),
                    )),
                }))
            }
        }
    }

    async fn cancel_by_token(&self, req: Request<v1::UserTokenRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        let (_, sid, provider_type) = try_grpc!(Jwt::verify::<i32>(
            jwt,
            &req.token,
            NAME,
            v1::UserTokenAction::Cancel.as_str_name()
        ))?;
        if let Some(provider_type) = provider_type {
            let provider_type =
                try_grpc!(v1::user_index_response::item::Type::try_from(provider_type))?;
            let user = match provider_type {
                v1::user_index_response::item::Type::Email => {
                    let it = try_grpc!(EmailUserDao::by_email(db, &sid))?;
                    try_grpc!(it.available())?;
                    Ok(it.user_id)
                }
                v1::user_index_response::item::Type::Google => {
                    let it = try_grpc!(GoogleUserDao::by_sub(db, &sid))?;
                    Ok(it.user_id)
                }
                v1::user_index_response::item::Type::WechatMiniProgram => {
                    let it = try_grpc!(WechatMiniProgramDao::by_union_id(db, &sid))?;
                    Ok(it.user_id)
                }
                v1::user_index_response::item::Type::WechatOauth2 => {
                    let it = try_grpc!(WechatOauth2Dao::by_union_id(db, &sid))?;
                    Ok(it.user_id)
                }
                v1::user_index_response::item::Type::Phone => {
                    // TODO
                    Err(Status::invalid_argument(
                        "cancel by phone not supported yet",
                    ))
                }
            }?;

            try_grpc!(db.transaction::<_, Error, _>(move |db| {
                UserDao::enable(db, user, false)?;
                LogDao::add::<_, User>(
                    db,
                    user,
                    NAME,
                    v1::user_logs_response::item::Level::Warn,
                    &ss.client_ip,
                    None,
                    format!("disable by self({})", provider_type.as_str_name()),
                )?;
                Ok(())
            }))?;
        }

        Ok(Response::new(()))
    }
    async fn disable(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            UserDao::enable(db, req.id, false)?;
            LogDao::add::<_, User>(
                db,
                req.id,
                NAME,
                v1::user_logs_response::item::Level::Warn,
                &ss.client_ip,
                None,
                "disable by administrator",
            )?;

            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn enable(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            UserDao::enable(db, req.id, true)?;
            LogDao::add::<_, User>(
                db,
                req.id,
                NAME,
                v1::user_logs_response::item::Level::Warn,
                &ss.client_ip,
                None,
                "enable by administrator",
            )?;

            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn lock(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            UserDao::lock(db, req.id, true)?;
            LogDao::add::<_, User>(
                db,
                req.id,
                NAME,
                v1::user_logs_response::item::Level::Warn,
                &ss.client_ip,
                None,
                "lock by administrator",
            )?;

            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn unlock(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            UserDao::lock(db, req.id, false)?;
            LogDao::add::<_, User>(
                db,
                req.id,
                NAME,
                v1::user_logs_response::item::Level::Warn,
                &ss.client_ip,
                None,
                "unlock by administrator",
            )?;

            Ok(())
        }))?;

        Ok(Response::new(()))
    }

    async fn confirm_email(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }
        let it = try_grpc!(EmailUserDao::by_id(db, req.id))?;
        if it.confirmed_at.is_none() {
            return Err(Status::resource_exhausted("user already confirmed"));
        }
        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            EmailUserDao::confirm(db, it.id)?;
            LogDao::add::<_, User>(
                db,
                it.user_id,
                NAME,
                v1::user_logs_response::item::Level::Warn,
                &ss.client_ip,
                None,
                "unlock by administrator",
            )?;

            Ok(())
        }))?;

        Ok(Response::new(()))
    }

    async fn set_password(&self, req: Request<v1::UserSetPasswordRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_hmac = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_hmac))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        {
            let form = SetPasswordForm {
                password: &req.password,
            };
            try_grpc!(form.validate())?;
        }

        let provider_type = try_grpc!(v1::user_index_response::item::Type::try_from(
            req.provider_type
        ))?;
        if provider_type == v1::user_index_response::item::Type::Email {
            try_grpc!(db.transaction::<_, Error, _>(move |db| {
                let it = EmailUserDao::by_id(db, req.provider_id)?;
                EmailUserDao::password(db, jwt_hmac, it.id, &req.password)?;

                LogDao::add::<_, User>(
                    db,
                    it.user_id,
                    NAME,
                    v1::user_logs_response::item::Level::Warn,
                    &ss.client_ip,
                    None,
                    "unlock by administrator",
                )?;

                Ok(())
            }))?;

            return Ok(Response::new(()));
        }
        Err(Status::invalid_argument(format!(
            "not supported for {} yest",
            provider_type.as_str_name()
        )))
    }

    async fn index_by_email(&self, req: Request<Pager>) -> GrpcResult<v1::UserIndexResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let pager = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let total = try_grpc!(EmailUserDao::count(db))?;
        let tmp = try_grpc!(EmailUserDao::all(db, pager.offset(total), pager.size()))?;
        let mut items = Vec::new();
        for iu in tmp.iter() {
            let user = try_grpc!(UserDao::by_id(db, iu.user_id))?;
            let it = user_details_by_email(&user, iu);
            items.push(it);
        }
        Ok(Response::new(v1::UserIndexResponse {
            items,
            pagination: Some(Pagination::new(&pager, total)),
        }))
    }
    async fn index_by_google(&self, req: Request<Pager>) -> GrpcResult<v1::UserIndexResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let pager = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let total = try_grpc!(GoogleUserDao::count(db))?;
        let tmp = try_grpc!(GoogleUserDao::all(db, pager.offset(total), pager.size()))?;
        let mut items = Vec::new();
        for iu in tmp.iter() {
            let user = try_grpc!(UserDao::by_id(db, iu.user_id))?;
            let it = user_details_by_google(&user, iu);
            items.push(it);
        }
        Ok(Response::new(v1::UserIndexResponse {
            items,
            pagination: Some(Pagination::new(&pager, total)),
        }))
    }
    async fn index_by_wechat_mini_program(
        &self,
        req: Request<Pager>,
    ) -> GrpcResult<v1::UserIndexResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let pager = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let total = try_grpc!(WechatMiniProgramDao::count(db))?;
        let tmp = try_grpc!(WechatMiniProgramDao::all(
            db,
            pager.offset(total),
            pager.size()
        ))?;
        let mut items = Vec::new();
        for iu in tmp.iter() {
            let user = try_grpc!(UserDao::by_id(db, iu.user_id))?;
            let it = user_details_by_wechat_mini_program(&user, iu);
            items.push(it);
        }
        Ok(Response::new(v1::UserIndexResponse {
            items,
            pagination: Some(Pagination::new(&pager, total)),
        }))
    }
    async fn index_by_wechat_oauth2(
        &self,
        req: Request<Pager>,
    ) -> GrpcResult<v1::UserIndexResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let pager = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let total = try_grpc!(WechatOauth2Dao::count(db))?;
        let tmp = try_grpc!(WechatOauth2Dao::all(db, pager.offset(total), pager.size()))?;
        let mut items = Vec::new();
        for iu in tmp.iter() {
            let user = try_grpc!(UserDao::by_id(db, iu.user_id))?;
            let it = user_details_by_wechat_oauth2(&user, iu);
            items.push(it);
        }
        Ok(Response::new(v1::UserIndexResponse {
            items,
            pagination: Some(Pagination::new(&pager, total)),
        }))
    }
}

#[derive(Validate)]
struct ProfileForm<'a> {
    #[validate(length(min = 2, max = 31))]
    real_name: &'a str,
    #[validate(url, length(max = 127))]
    avatar: &'a str,
}

#[derive(Validate)]
struct LocationForm<'a> {
    #[validate(length(min = 2, max = 15))]
    lang: &'a str,
    #[validate(length(min = 2, max = 32))]
    timezone: &'a str,
}

#[derive(Validate)]
struct ChangePasswordForm<'a> {
    #[validate(length(min = 1))]
    current_password: &'a str,
    #[validate(length(min = 6, max = 32))]
    new_password: &'a str,
}

#[derive(Validate)]
pub struct ResetPasswordForm<'a> {
    #[validate(length(min = 6, max = 32))]
    pub password: &'a str,
    #[validate(length(max = 255))]
    pub token: &'a str,
}

#[derive(Validate)]
pub struct SetPasswordForm<'a> {
    #[validate(length(min = 6, max = 32))]
    pub password: &'a str,
}

#[derive(Validate)]
struct SignUpByEmailForm<'a> {
    #[validate(length(min = 2, max = 31))]
    nickname: &'a str,
    #[validate(length(min = 2, max = 31))]
    real_name: &'a str,
    #[validate(email, length(min = 6, max = 127))]
    email: &'a str,
    #[validate(length(min = 6, max = 31))]
    password: &'a str,
    #[validate(length(min = 3, max = 31))]
    timezone: &'a str,
    #[validate(url, length(min = 6, max = 64))]
    home: &'a str,
}

#[derive(Validate)]
struct HomeForm<'a> {
    #[validate(url, length(min = 6, max = 64))]
    home: &'a str,
}

impl From<Log> for v1::user_logs_response::Item {
    fn from(x: Log) -> Self {
        Self {
            id: x.id,
            ip: x.ip.clone(),
            message: x.message.clone(),
            resource: Some(v1::Resource {
                r#type: x.resource_type.clone(),
                id: x.resource_id,
            }),
            level: x.level,
            created_at: Some(to_timestamp!(x.created_at)),
        }
    }
}
fn email_user_by_from_request(db: &mut Db, req: &v1::UserEmailRequest) -> Result<EmailUser> {
    match req.user {
        Some(v1::user_email_request::User::Email(ref email)) => {
            let email = to_code!(email);
            EmailUserDao::by_email(db, &email)
        }
        Some(v1::user_email_request::User::Nickname(ref nickname)) => {
            let nickname = to_code!(nickname);
            EmailUserDao::by_nickname(db, &nickname)
        }
        None => Err(Box::new(HttpError(
            StatusCode::NOT_FOUND,
            Some("empty user".to_string()),
        ))),
    }
}
fn user_from_sign_in_by_password_request<S: Password>(
    db: &mut Db,
    hmac: &S,
    req: &v1::UserSignInByPasswordRequest,
) -> Result<(User, v1::user_index_response::Item, i64)> {
    match req.who {
        Some(v1::user_sign_in_by_password_request::Who::Email(ref email)) => {
            let email = to_code!(email);
            let it = EmailUserDao::by_email(db, &email)?;
            it.available()?;
            it.auth(hmac, &req.password)?;
            let user = UserDao::by_id(db, it.user_id)?;
            user.available()?;
            let detail = user_details_by_email(&user, &it);
            Ok((user, detail, it.id))
        }
        Some(v1::user_sign_in_by_password_request::Who::Nickname(ref nickname)) => {
            let nickname = to_code!(nickname);
            let it = EmailUserDao::by_nickname(db, &nickname)?;
            it.available()?;
            it.auth(hmac, &req.password)?;
            let user = UserDao::by_id(db, it.user_id)?;
            user.available()?;
            let detail = user_details_by_email(&user, &it);
            Ok((user, detail, it.id))
        }
        _ => Err(Box::new(HttpError(
            StatusCode::NOT_FOUND,
            Some("empty user".to_string()),
        ))),
    }
}
fn user_details_by_email(user: &User, it: &EmailUser) -> v1::user_index_response::Item {
    v1::user_index_response::Item {
        r#type: v1::user_index_response::item::Type::Email as i32,
        name: Some(it.real_name.clone()),
        id: it.email.clone(),
        lang: user.lang.clone(),
        timezone: user.timezone.clone(),
        avatar: Some(it.avatar.clone()),
    }
}
fn user_details_by_google(user: &User, it: &GoogleUser) -> v1::user_index_response::Item {
    v1::user_index_response::Item {
        r#type: v1::user_index_response::item::Type::Google as i32,
        name: it.name.clone(),
        id: it.sub.clone(),
        lang: user.lang.clone(),
        timezone: user.timezone.clone(),
        avatar: it.picture.clone(),
    }
}
fn user_details_by_wechat_mini_program(
    user: &User,
    it: &WechatMiniProgramUser,
) -> v1::user_index_response::Item {
    v1::user_index_response::Item {
        r#type: v1::user_index_response::item::Type::WechatMiniProgram as i32,
        name: it.nickname.clone(),
        id: it.union_id.clone(),
        lang: user.lang.clone(),
        timezone: user.timezone.clone(),
        avatar: it.avatar_url.clone(),
    }
}
fn user_details_by_wechat_oauth2(
    user: &User,
    it: &WechatOauth2User,
) -> v1::user_index_response::Item {
    v1::user_index_response::Item {
        r#type: v1::user_index_response::item::Type::WechatOauth2 as i32,
        name: Some(it.nickname.clone()),
        id: it.union_id.clone(),
        lang: user.lang.clone(),
        timezone: user.timezone.clone(),
        avatar: it.head_img_url.clone(),
    }
}

async fn send_email<J: Jwt>(
    db: &mut Db,
    jwt: &J,
    queue: &RabbitMq,
    lang: &str,
    (name, email): (&str, &str),
    home: &str,
    action: v1::UserTokenAction,
) -> Result<()> {
    let token = Jwt::sign_by_duration(
        jwt,
        NAME,
        email,
        action.as_str_name(),
        &Some(v1::user_index_response::item::Type::Email as i32),
        Duration::hours(1),
    )?;
    let subject = I18n::t(
        db,
        lang,
        &format!("users.email.{}.subject", action.as_str_name()),
        &Some(HashMap::from([("username", name)])),
    );
    let body = I18n::t(
        db,
        lang,
        &format!("users.email.{}.body", action.as_str_name()),
        &Some(HashMap::from([
            ("username", name),
            ("home", home),
            ("token", &token),
        ])),
    );
    let task = EmailSendTask {
        to: EmailAddress {
            name: name.to_string(),
            email: email.to_string(),
        },
        subject,
        body: EmailBody {
            text: body,
            html: true,
        },
        cc: BTreeSet::new(),
        bcc: BTreeSet::new(),
        attachments: BTreeSet::new(),
    };
    let queue = Amqp::open(queue).await?;
    ThriftAmqpProtocol::produce(&queue, &task).await?;
    Ok(())
}
async fn send_sms<J: Jwt>(
    db: &mut Db,
    jwt: &J,
    queue: &RabbitMq,
    lang: &str,
    (name, phone): (&str, &str),
    home: &str,
    action: v1::UserTokenAction,
) -> Result<()> {
    let token = Jwt::sign_by_duration(
        jwt,
        NAME,
        phone,
        action.as_str_name(),
        &Some(v1::user_index_response::item::Type::Phone as i32),
        Duration::hours(1),
    )?;
    let body = I18n::t(
        db,
        lang,
        &format!("users.sms.{}.body", action.as_str_name()),
        &Some(HashMap::from([("username", name)])),
    );
    let task = SmsSendTask {
        to: BTreeSet::from([phone.to_string()]),
        body,
        callback: Some(format!("{home}/twilio/callbacks/sms-reply/{token}")),
    };
    let queue = Amqp::open(queue).await?;
    ThriftAmqpProtocol::produce(&queue, &task).await?;
    Ok(())
}

fn new_sign_in_response<J: Jwt, P: Policy>(
    db: &mut Db,
    jwt: &J,
    policy: &P,
    (user, details, provider_id): (&User, v1::user_index_response::Item, i64),
    (ip, ttl): (&str, Duration),
) -> Result<v1::UserSignInResponse> {
    let sid = uuid();
    let provider_type = v1::user_index_response::item::Type::try_from(details.r#type)?;
    {
        let did = details.id.clone();
        db.transaction::<_, Error, _>(move |db| {
            SessionDao::create(db, user.id, provider_type, provider_id, ip, ttl)?;
            LogDao::add::<_, User>(
                db,
                user.id,
                NAME,
                v1::user_logs_response::item::Level::Info,
                ip,
                None,
                format!("sign in by ({}, {})", provider_type.as_str_name(), &did),
            )?;

            Ok(())
        })?;
    }
    let token = Jwt::sign_by_duration::<i32>(
        jwt,
        NAME,
        &sid,
        v1::UserTokenAction::SignIn.as_str_name(),
        &None,
        ttl,
    )?;
    let roles = {
        let items = policy.get_implicit_roles_for_user(user.id)?;
        items.into_iter().collect()
    };
    let permissions = {
        let mut items = Vec::new();
        for pm in policy.get_implicit_permissions_for_user(user.id)?.iter() {
            items.push(v1::Permission {
                operation: pm.operation.clone(),
                resource: Some(v1::Resource {
                    r#type: pm.resource.type_.clone(),
                    id: pm.resource.id,
                }),
            });
        }
        items
    };
    Ok(v1::UserSignInResponse {
        token,
        user: Some(details),
        menus: Vec::new(),
        roles,
        permissions,
    })
}
