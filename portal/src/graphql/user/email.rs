use std::any::type_name;

use chrono::Duration;
use diesel::Connection as DieselConnection;
use flatbuffers::FlatBufferBuilder;
use hyacinth::email_v1::{
    Address as EmailAddress, AddressArgs as EmailAddressArgs, Body as EmailBody,
    BodyArgs as EmailBodyArgs, Task as EmailTask, TaskArgs as EmailTaskArgs,
};
use hyper::StatusCode;
use juniper::GraphQLInputObject;
use lapin::options::BasicPublishOptions;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::super::super::{
    Error, HttpError, Jwt, PasswordHashing, Result,
    cache::redis::StandaloneConnection as Cache,
    content_types::APPLICATION_X_FLATBUFFERS,
    models::{
        locale::I18n,
        log::{Dao as LogDao, Level},
        user::{
            Dao as UserDao, Type as UserType,
            email::{Dao as EmailUserDao, Item as EmailUserItem},
        },
    },
    orm::postgresql::Connection as Db,
    queue::rabbitmq::Client as RabbitMq,
    rbac::Rbac,
};
use super::super::{Plugin, Session};
use super::{CurrentUser, SignInResponse, TokenPayload};

#[derive(Clone, Debug, Validate)]
pub struct SetPassword {
    pub id: i32,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
}

impl SetPassword {
    pub async fn execute<R: Rbac, J: Jwt, H: PasswordHashing>(
        &self,
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        hashing: &H,
    ) -> Result<()> {
        self.validate()?;

        let current_user = ss.current_user(db, cache, jwt).await?;
        rbac.is_administrator(current_user.id()).await?;

        let it = {
            let it = EmailUserDao::by_id(db, self.id as i64)?;
            if rbac.is_root(it.user_id).await.is_ok() {
                return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
            }
            it
        };
        let password = hashing.sign(&self.password).await?;
        let ip = ss.client_ip();

        db.transaction::<_, Error, _>(|tx| {
            EmailUserDao::set_password(tx, it.id, &password)?;

            LogDao::create::<Plugin, _>(
                tx,
                it.user_id,
                Level::Info,
                ip,
                format!("Reset password by administrator {}.", current_user.uid()),
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Clone, Debug, Validate, GraphQLInputObject)]
#[graphql(name = "UserSignUpByEmailRequest")]
pub struct SignUp {
    #[validate(length(min = 1, max = 31))]
    pub name: String,
    #[validate(length(min = 5, max = 31), email)]
    pub email: String,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
    #[validate(length(min = 2, max = 7))]
    pub lang: String,
    #[validate(length(min = 3, max = 31))]
    pub timezone: String,
}

impl SignUp {
    pub fn create(&self, db: &mut Db, password: &str) -> Result<()> {
        self.validate()?;

        let it = {
            let uid = Uuid::new_v4().to_string();
            UserDao::create(db, &uid, &self.lang.parse()?, self.timezone.parse()?)?;
            let user = UserDao::by_uid(db, &uid)?;
            EmailUserDao::create(db, user.id, &self.name, &self.email, password)?;
            EmailUserDao::by_email(db, &self.email)?
        };
        UserDao::set_name(db, it.user_id, Some(it.name.as_str()))?;
        UserDao::set_avatar(db, it.user_id, Some(it.avatar.as_str()))?;
        Ok(())
    }

    pub async fn execute<J: Jwt, H: PasswordHashing>(
        &self,
        ss: &Session,
        db: &mut Db,
        queue: &RabbitMq,
        jwt: &J,
        hashing: &H,
    ) -> Result<()> {
        let password = hashing.sign(&self.password).await?;
        let ip = ss.client_ip();

        let it = db.transaction::<_, Error, _>(|tx| {
            self.create(tx, &password)?;
            let it = EmailUserDao::by_email(tx, &self.email)?;
            LogDao::create::<Plugin, _>(tx, it.user_id, Level::Info, ip, "Sign up.")?;
            Ok(it)
        })?;

        send_email(db, queue, jwt, &it, Confirm::ACTION).await?;
        Ok(())
    }
}

#[derive(Debug, Validate)]
pub struct SignIn {
    #[validate(length(min = 5, max = 31), email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl SignIn {
    pub async fn execute<R: Rbac, J: Jwt, H: PasswordHashing>(
        &self,
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        hashing: &H,
    ) -> Result<SignInResponse> {
        let it = EmailUserDao::by_email(db, &self.email)?;
        hashing.verify(&it.password, &self.password).await?;
        it.is_enable()?;

        let ip = ss.client_ip();

        db.transaction::<_, Error, _>(|tx| {
            UserDao::sign_in(tx, it.user_id, ip)?;
            LogDao::create::<Plugin, _>(tx, it.user_id, Level::Info, ip, "Sign in by email.")?;
            Ok(())
        })?;

        SignInResponse::new(db, cache, rbac, jwt, it.user_id, UserType::Email, &it.email).await
    }
}

#[derive(Debug, Validate)]
pub struct Confirm {
    #[validate(length(min = 5, max = 31), email)]
    pub email: String,
}

impl Confirm {
    const ACTION: &str = "user.confirm-by-email";

    pub async fn execute<J: Jwt>(&self, db: &mut Db, queue: &RabbitMq, jwt: &J) -> Result<()> {
        self.validate()?;
        let it = EmailUserDao::by_email(db, &self.email)?;
        Self::can(&it)?;
        send_email(db, queue, jwt, &it, Self::ACTION).await?;
        Ok(())
    }

    fn can(it: &EmailUserItem) -> Result<()> {
        if it.confirmed_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::PRECONDITION_FAILED,
                Some("User is already confirmed".to_string()),
            )));
        }
        if it.locked_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::LOCKED,
                Some("User is locked".to_string()),
            )));
        }
        if it.deleted_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::GONE,
                Some("User is gone".to_string()),
            )));
        }
        Ok(())
    }
}

pub async fn confirm<J: Jwt>(ss: &Session, db: &mut Db, jwt: &J, token: &str) -> Result<()> {
    let (subject, payload) = jwt
        .verify::<TokenPayload>(token, CurrentUser::ISSUER, Confirm::ACTION)
        .await?;
    {
        let payload = payload.ok_or_else(|| {
            Box::new(HttpError(
                StatusCode::FORBIDDEN,
                Some("Invalid token".to_string()),
            ))
        })?;
        if payload.r#type != UserType::Email {
            return Err(Box::new(HttpError(
                StatusCode::FORBIDDEN,
                Some("Invalid token".to_string()),
            )));
        }
    }
    let it = EmailUserDao::by_email(db, &subject)?;
    Confirm::can(&it)?;

    let ip = ss.client_ip();

    db.transaction::<_, Error, _>(|tx| {
        EmailUserDao::confirm(tx, it.user_id)?;
        LogDao::create::<Plugin, _>(tx, it.user_id, Level::Info, ip, "Confirmed by email.")?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Debug, Validate)]
pub struct Unlock {
    #[validate(length(min = 5, max = 31), email)]
    pub email: String,
}

impl Unlock {
    const ACTION: &str = "user.unlock-by-email";

    pub async fn execute<J: Jwt>(&self, db: &mut Db, queue: &RabbitMq, jwt: &J) -> Result<()> {
        self.validate()?;
        let it = EmailUserDao::by_email(db, &self.email)?;
        Self::can(&it)?;
        send_email(db, queue, jwt, &it, Self::ACTION).await?;
        Ok(())
    }

    fn can(it: &EmailUserItem) -> Result<()> {
        if it.confirmed_at.is_none() {
            return Err(Box::new(HttpError(
                StatusCode::PRECONDITION_FAILED,
                Some("User isn't confirmed yet".to_string()),
            )));
        }
        if it.locked_at.is_none() {
            return Err(Box::new(HttpError(
                StatusCode::PRECONDITION_FAILED,
                Some("User isn't locked yet".to_string()),
            )));
        }
        if it.deleted_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::GONE,
                Some("User is gone".to_string()),
            )));
        }
        Ok(())
    }
}

pub async fn unlock<J: Jwt>(ss: &Session, db: &mut Db, jwt: &J, token: &str) -> Result<()> {
    let (subject, payload) = jwt
        .verify::<TokenPayload>(token, CurrentUser::ISSUER, Unlock::ACTION)
        .await?;
    {
        let payload = payload.ok_or_else(|| {
            Box::new(HttpError(
                StatusCode::FORBIDDEN,
                Some("Invalid token".to_string()),
            ))
        })?;
        if payload.r#type != UserType::Email {
            return Err(Box::new(HttpError(
                StatusCode::FORBIDDEN,
                Some("Invalid token".to_string()),
            )));
        }
    }
    let it = EmailUserDao::by_email(db, &subject)?;
    Unlock::can(&it)?;

    let ip = ss.client_ip();

    db.transaction::<_, Error, _>(|tx| {
        EmailUserDao::unlock(tx, it.user_id)?;
        LogDao::create::<Plugin, _>(tx, it.user_id, Level::Info, ip, "Unlock by email.")?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Debug, Validate)]
pub struct ForgotPassword {
    #[validate(length(min = 5, max = 31), email)]
    pub email: String,
}

impl ForgotPassword {
    pub async fn execute<J: Jwt>(&self, db: &mut Db, queue: &RabbitMq, jwt: &J) -> Result<()> {
        self.validate()?;
        let it = EmailUserDao::by_email(db, &self.email)?;
        it.is_enable()?;
        send_email(db, queue, jwt, &it, ResetPassword::ACTION).await?;
        Ok(())
    }
}

#[derive(Clone, Debug, Validate)]
pub struct ResetPassword {
    #[validate(length(min = 1))]
    pub token: String,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
}

impl ResetPassword {
    const ACTION: &str = "user.reset-password-by-email";

    pub async fn execute<J: Jwt, H: PasswordHashing>(
        &self,
        ss: &Session,
        db: &mut Db,
        jwt: &J,
        hashing: &H,
    ) -> Result<()> {
        self.validate()?;
        let (subject, payload) = jwt
            .verify::<TokenPayload>(&self.token, CurrentUser::ISSUER, Self::ACTION)
            .await?;
        {
            let payload = payload.ok_or_else(|| {
                Box::new(HttpError(
                    StatusCode::FORBIDDEN,
                    Some("Invalid token".to_string()),
                ))
            })?;
            if payload.r#type != UserType::Email {
                return Err(Box::new(HttpError(
                    StatusCode::FORBIDDEN,
                    Some("Invalid token".to_string()),
                )));
            }
        }
        let it = EmailUserDao::by_email(db, &subject)?;
        it.is_enable()?;

        let ip = ss.client_ip();
        let password = hashing.sign(&self.password).await?;

        db.transaction::<_, Error, _>(|tx| {
            EmailUserDao::set_password(tx, it.user_id, &password)?;
            LogDao::create::<Plugin, _>(
                tx,
                it.user_id,
                Level::Info,
                ip,
                "Reset password by email.",
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

async fn send_email<J: Jwt>(
    db: &mut Db,
    queue: &RabbitMq,
    jwt: &J,
    item: &EmailUserItem,
    action: &str,
) -> Result<()> {
    let user = UserDao::by_id(db, item.user_id)?;
    user.is_enable()?;
    let lang = user.lang.parse()?;

    let args = EmailTemplate {
        name: item.name.clone(),
        token: jwt
            .sign(
                CurrentUser::ISSUER,
                &item.email,
                vec![action],
                Duration::minutes(15),
                Some(TokenPayload {
                    r#type: UserType::Email,
                }),
            )
            .await?,
    };
    let subject = I18n::t(
        db,
        &lang,
        &format!("portal.emails.{}.subject", action),
        Some(&args),
    );
    let body = I18n::t(
        db,
        &lang,
        &format!("portal.emails.{}.body", action),
        Some(&args),
    );

    let mut builder = FlatBufferBuilder::new();
    {
        let subject = builder.create_string(&subject);
        let body_content = builder.create_string(&body);
        let to_name = builder.create_string(&item.name);
        let to_email = builder.create_string(&item.email);

        let body = EmailBody::create(
            &mut builder,
            &EmailBodyArgs {
                html: true,
                content: Some(body_content),
            },
        );

        let to = EmailAddress::create(
            &mut builder,
            &EmailAddressArgs {
                name: Some(to_name),
                email: Some(to_email),
            },
        );

        let task = EmailTask::create(
            &mut builder,
            &EmailTaskArgs {
                to: Some(to),
                subject: Some(subject),
                body: Some(body),
                ..Default::default()
            },
        );

        builder.finish(task, None);
    }
    let task: &[u8] = builder.finished_data();

    queue
        .publish(
            "",
            type_name::<EmailTask>(),
            APPLICATION_X_FLATBUFFERS,
            task,
            BasicPublishOptions::default(),
        )
        .await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct EmailTemplate {
    name: String,
    token: String,
}
