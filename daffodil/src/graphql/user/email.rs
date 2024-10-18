use std::collections::BTreeMap;
use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use chrono::{Duration, NaiveDateTime};
use chrono_tz::Tz;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use language_tags::LanguageTag;
use petunia::{
    daisy::v1::{
        email_task::{Address, Body as EmailBody},
        EmailTask,
    },
    graphql::{Pager, Pagination},
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    orm::postgresql::{Connection as Db, Pool as DbPool},
    queue::amqp::{Protobuf as ProtobufQueue, RabbitMq},
    session::Session,
    Error, HttpError, Result,
};
use tokio::sync::Mutex;
use uuid::Uuid;
use validator::Validate;

use super::super::super::models::{
    locale::I18n,
    log::{Dao as LogDao, Level as LogLevel},
    session::ProviderType,
    user::{
        email::{Dao as EmailDao, Item as EmailUser},
        Action as UserAction, Dao as UserDao, Item as User,
    },
};
use super::super::NAME;
use super::SignInResponse;

#[derive(Validate)]
pub struct SignUp {
    #[validate(length(min = 2, max = 31))]
    pub real_name: String,
    #[validate(length(min = 2, max = 31))]
    pub nickname: String,
    #[validate(email, length(min = 6, max = 63))]
    pub email: String,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
    pub timezone: String,
    pub lang: String,
}

impl SignUp {
    pub async fn execute(
        &self,
        db: &DbPool,
        jwt: &Jwt,
        queue: &RabbitMq,
        client_ip: &str,
    ) -> Result<()> {
        self.validate()?;

        let lang = LanguageTag::from_str(&self.lang)?;
        let timezone = Tz::from_str(&self.timezone)?;
        let uid = Uuid::new_v4().to_string();

        let mut db = db.get()?;
        let db = db.deref_mut();

        db.transaction::<_, Error, _>(|db| {
            if EmailDao::by_email(db, &self.email).is_ok() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some(format!("Email user({}) is already exists.", self.email)),
                )));
            }
            if EmailDao::by_nickname(db, &self.nickname).is_ok() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some(format!("Email user({}) is already exists.", self.nickname)),
                )));
            }

            UserDao::create(db, &uid, &lang, timezone)?;
            let user = UserDao::by_uid(db, &uid)?;
            EmailDao::create(
                db,
                user.id,
                &self.real_name,
                &self.nickname,
                &self.email,
                &self.password,
            )?;
            LogDao::create::<_, EmailUser>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                client_ip,
                None,
                "Sign up.",
            )?;
            Ok(user)
        })?;
        send_email(
            (&self.email, &self.real_name),
            &self.lang,
            db,
            jwt,
            queue,
            UserAction::Confirm,
        )
        .await
    }
}

#[derive(Validate)]
pub struct SignIn {
    #[validate(length(min = 2))]
    pub user: String,
    #[validate(length(min = 6))]
    pub password: String,
}

impl SignIn {
    pub async fn execute(
        &self,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        client_ip: &str,
    ) -> Result<SignInResponse> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let eu = {
            let eu = by_nick_or_email(db, &self.user)?;

            if eu.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("user is disabled".to_string()),
                )));
            }
            if eu.confirmed_at.is_none() {
                return Err(Box::new(HttpError(
                    StatusCode::PRECONDITION_REQUIRED,
                    Some("user isn't confirmed".to_string()),
                )));
            }
            eu
        };

        SignInResponse::new::<EmailUser>(
            (eu.user_id, eu.real_name.clone(), ProviderType::Email, eu.id),
            db,
            jwt,
            enforcer,
            client_ip,
        )
        .await
    }
}

fn by_nick_or_email(db: &mut Db, user: &str) -> Result<EmailUser> {
    if let Ok(it) = EmailDao::by_nickname(db, user) {
        return Ok(it);
    }
    EmailDao::by_email(db, user)
}

async fn send_email(
    (email, real_name): (&str, &str),
    lang: &str,
    db: &mut Db,
    jwt: &Jwt,
    queue: &RabbitMq,
    action: UserAction,
) -> Result<()> {
    let action = action.to_string();
    let ttl = Duration::hours(1);
    let (nbf, exp) = Jwt::timestamps(ttl);
    let token = jwt.sign(email, &action, nbf, exp)?;

    let mut args = BTreeMap::new();
    {
        args.insert("real_name", real_name.to_string());
        args.insert("token", token);
        args.insert("action", action.clone());
    }
    let subject = I18n::t(
        db,
        lang,
        &format!("{}.mailers.user.{}.subject", NAME, action),
        Some(&args),
    );
    let body = I18n::t(
        db,
        lang,
        &format!("{}.mailers.user.{}.body", NAME, action),
        Some(&args),
    );
    ProtobufQueue::produce(
        queue,
        &EmailTask {
            subject,
            body: Some(EmailBody {
                html: true,
                text: body,
            }),
            to: Some(Address {
                email: email.to_string(),
                name: real_name.to_string(),
            }),
            cc: Vec::new(),
            bcc: Vec::new(),
            attachments: Vec::new(),
        },
    )
    .await?;
    Ok(())
}

#[derive(Validate)]
pub struct Email {
    #[validate(length(min = 2, max = 31))]
    pub user: String,
}

impl Email {
    pub async fn forgot_password(&self, db: &DbPool, jwt: &Jwt, queue: &RabbitMq) -> Result<()> {
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();

        let eu = by_nick_or_email(db, &self.user)?;
        {
            if eu.confirmed_at.is_none() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) isn't confirmed yet.", self.user)),
                )));
            }
            if eu.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", self.user)),
                )));
            }
        }
        let user = UserDao::by_id(db, eu.user_id)?;
        {
            if user.locked_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is locked.", self.user)),
                )));
            }
            if user.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", self.user)),
                )));
            }
        }

        send_email(
            (&eu.email, &eu.real_name),
            &user.lang,
            db,
            jwt,
            queue,
            UserAction::ResetPassword,
        )
        .await
    }
    pub async fn unlock(&self, db: &DbPool, jwt: &Jwt, queue: &RabbitMq) -> Result<()> {
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();

        let eu = by_nick_or_email(db, &self.user)?;
        {
            if eu.confirmed_at.is_none() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is not confirmed yet.", self.user)),
                )));
            }
            if eu.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", self.user)),
                )));
            }
        }
        let user = UserDao::by_id(db, eu.user_id)?;
        {
            if user.locked_at.is_none() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) isn't locked.", self.user)),
                )));
            }
            if user.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", self.user)),
                )));
            }
        }

        send_email(
            (&eu.email, &eu.real_name),
            &user.lang,
            db,
            jwt,
            queue,
            UserAction::Unlock,
        )
        .await
    }
    pub async fn confirm(&self, db: &DbPool, jwt: &Jwt, queue: &RabbitMq) -> Result<()> {
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();

        let eu = by_nick_or_email(db, &self.user)?;
        {
            if eu.confirmed_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is already confirmed.", self.user)),
                )));
            }
            if eu.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", self.user)),
                )));
            }
        }
        let user = UserDao::by_id(db, eu.user_id)?;
        {
            if user.locked_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is locked.", self.user)),
                )));
            }
            if user.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", self.user)),
                )));
            }
        }

        send_email(
            (&eu.email, &eu.real_name),
            &user.lang,
            db,
            jwt,
            queue,
            UserAction::Confirm,
        )
        .await
    }
}

#[derive(Validate)]
pub struct Token {
    #[validate(length(min = 32, max = 255))]
    pub token: String,
}

impl Token {
    pub fn unlock(&self, db: &DbPool, jwt: &Jwt, client_ip: &str) -> Result<()> {
        self.validate()?;

        let email = jwt.verify(&self.token, &UserAction::Unlock.to_string())?;

        let mut db = db.get()?;
        let db = db.deref_mut();

        let eu = EmailDao::by_email(db, &email)?;
        {
            if eu.confirmed_at.is_none() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) isn't confirmed yet.", email)),
                )));
            }
            if eu.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", email)),
                )));
            }
        }
        let user = UserDao::by_id(db, eu.user_id)?;
        {
            if user.locked_at.is_none() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) isn't locked.", email)),
                )));
            }
            if user.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", email)),
                )));
            }
        }

        db.transaction::<_, Error, _>(|db| {
            UserDao::unlock(db, user.id)?;
            LogDao::create::<_, EmailUser>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                client_ip,
                None,
                "Unlock by email.",
            )?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn confirm(&self, db: &DbPool, jwt: &Jwt, client_ip: &str) -> Result<()> {
        self.validate()?;

        let email = jwt.verify(&self.token, &UserAction::Confirm.to_string())?;

        let mut db = db.get()?;
        let db = db.deref_mut();

        let eu = EmailDao::by_email(db, &email)?;
        {
            if eu.confirmed_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is already confirmed.", email)),
                )));
            }
            if eu.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", email)),
                )));
            }
        }
        let user = UserDao::by_id(db, eu.user_id)?;
        {
            if user.locked_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is locked.", email)),
                )));
            }
            if user.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", email)),
                )));
            }
        }

        db.transaction::<_, Error, _>(|db| {
            EmailDao::confirm(db, eu.id)?;
            LogDao::create::<_, EmailUser>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                client_ip,
                None,
                "Confirm by email.",
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate)]
pub struct ResetPassword {
    #[validate(length(min = 32, max = 255))]
    pub token: String,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
}

impl ResetPassword {
    pub fn execute(&self, db: &DbPool, jwt: &Jwt, client_ip: &str) -> Result<()> {
        self.validate()?;

        let email = jwt.verify(&self.token, &UserAction::ResetPassword.to_string())?;

        let mut db = db.get()?;
        let db = db.deref_mut();

        let eu = EmailDao::by_email(db, &email)?;
        {
            if eu.confirmed_at.is_none() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) isn't confirmed yet.", email)),
                )));
            }
            if eu.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", email)),
                )));
            }
        }
        let user = UserDao::by_id(db, eu.user_id)?;
        {
            if user.locked_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is locked.", email)),
                )));
            }
            if user.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_GATEWAY,
                    Some(format!("Email user({}) is disabled.", email)),
                )));
            }
        }

        db.transaction::<_, Error, _>(|db| {
            EmailDao::set_password(db, eu.id, &self.password)?;
            LogDao::create::<_, EmailUser>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                client_ip,
                None,
                "Reset password by email.",
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "EmailUser")]
pub struct Item {
    pub id: i32,
    pub detail: super::Item,
    pub real_name: String,
    pub nickname: String,
    pub email: String,
    pub avatar: String,
    pub confirmed_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLObject)]
#[graphql(name = "EmailUserList")]
pub struct List {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

impl List {
    pub async fn new(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        pager: &Pager,
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let user = User::new(ss, db, jwt)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            user.is_administrator(enf)?;
        }
        let mut items = Vec::new();
        let total = EmailDao::total(db)?;
        let pagination = Pagination::new(pager, total);
        for eu in EmailDao::index(db, pager.offset(total), pager.size())?.iter() {
            let it = UserDao::by_id(db, eu.user_id)?;
            items.push(Item {
                id: eu.id,
                detail: it.into(),
                real_name: eu.real_name.clone(),
                nickname: eu.nickname.clone(),
                email: eu.email.clone(),
                avatar: eu.avatar.clone(),
                confirmed_at: eu.confirmed_at,
                deleted_at: eu.deleted_at,
                updated_at: eu.updated_at,
            });
        }
        Ok(Self { items, pagination })
    }
}

pub async fn confirm(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    client_ip: &str,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let user = User::new(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }
    db.transaction::<_, Error, _>(|db| {
        let eu = EmailDao::by_id(db, id)?;
        if eu.confirmed_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::BAD_GATEWAY,
                Some(format!("Email user({}) is already confirmed.", eu)),
            )));
        }
        EmailDao::confirm(db, eu.id)?;
        LogDao::create::<_, EmailUser>(
            db,
            eu.user_id,
            NAME,
            LogLevel::Info,
            client_ip,
            None,
            "Confirm by administrator.",
        )?;
        Ok(())
    })?;

    Ok(())
}
pub async fn enable(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    client_ip: &str,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let user = User::new(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }
    let eu = EmailDao::by_id(db, id)?;
    if eu.deleted_at.is_none() {
        return Err(Box::new(HttpError(
            StatusCode::BAD_GATEWAY,
            Some(format!("Email user({}) is already enabled.", eu)),
        )));
    }
    {
        let it = UserDao::by_id(db, eu.user_id)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        if it.is_root(enf).is_ok() {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
    }
    db.transaction::<_, Error, _>(|db| {
        EmailDao::enable(db, eu.id)?;
        LogDao::create::<_, EmailUser>(
            db,
            eu.user_id,
            NAME,
            LogLevel::Info,
            client_ip,
            None,
            "Enable by administrator.",
        )?;
        Ok(())
    })?;

    Ok(())
}
pub async fn disable(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    client_ip: &str,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let user = User::new(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }
    let eu = EmailDao::by_id(db, id)?;
    if eu.deleted_at.is_some() {
        return Err(Box::new(HttpError(
            StatusCode::BAD_GATEWAY,
            Some(format!("Email user({}) is already disabled.", eu)),
        )));
    }
    {
        let it = UserDao::by_id(db, eu.user_id)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        if it.is_root(enf).is_ok() {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
    }
    db.transaction::<_, Error, _>(|db| {
        EmailDao::disable(db, eu.id)?;
        LogDao::create::<_, EmailUser>(
            db,
            eu.user_id,
            NAME,
            LogLevel::Info,
            client_ip,
            None,
            "Disable by administrator.",
        )?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Validate)]
pub struct SetPassword {
    pub id: i32,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
}

impl SetPassword {
    pub async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        client_ip: &str,
    ) -> Result<()> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let user = User::new(ss, db, jwt)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            user.is_administrator(enf)?;
        }
        let eu = EmailDao::by_id(db, self.id)?;
        {
            let it = UserDao::by_id(db, eu.user_id)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            if it.is_root(enf).is_ok() {
                return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
            }
        }
        db.transaction::<_, Error, _>(|db| {
            EmailDao::set_password(db, eu.id, &self.password)?;
            LogDao::create::<_, EmailUser>(
                db,
                eu.user_id,
                NAME,
                LogLevel::Info,
                client_ip,
                None,
                "Change password by administrator.",
            )?;
            Ok(())
        })?;

        Ok(())
    }
}
