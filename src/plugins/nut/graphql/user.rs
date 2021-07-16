use std::collections::HashMap;

use chrono::Duration;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::super::{
    crypto::{Hmac, Password},
    i18n::I18n,
    jwt::Jwt,
    mailer::Email as EmailTask,
    orm::postgresql::Connection as Db,
    queue::amqp::RabbitMq,
    Error, HttpError, MediaType, Result,
};
use super::super::models::{
    log::{Dao as LogDao, Item as Log},
    operation::{Dao as OperationDao, Item as Operation},
    policy::{Dao as PolicyDao, Entity as Policy, Item as PolicyItem},
    resource::{Dao as ResouceDao, Item as Resource},
    role::Dao as RoleDao,
    user::{Dao as UserDao, Entity as UserEntity, Item as User, Token},
};
use super::{Page, Pagination, Session};

#[derive(GraphQLObject)]
#[graphql(description = "Current user information")]
pub struct CurrentUser {
    pub user: UserEntity,
    pub policies: Vec<Policy>,
}

impl CurrentUser {
    pub fn new(ss: &Session, db: &Db, jwt: &Jwt) -> Result<Self> {
        let user = ss.current_user(db, jwt)?;
        Ok(Self {
            policies: PolicyDao::entities(db, user.id)?,
            user: user.into(),
        })
    }
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "UserSignUpForm")]
pub struct SignUpForm {
    #[validate(length(min = 2))]
    pub nick_name: String,
    #[validate(length(min = 2))]
    pub real_name: String,
    #[validate(email, length(min = 2))]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1))]
    pub home: String,
}

impl SignUpForm {
    pub fn init_administrator(&self, ss: &Session, db: &Db, hmac: &Hmac) -> Result<()> {
        self.validate()?;
        if UserDao::count(db)? > 0 {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("Database isn't empty!".to_string()),
            )));
        }
        let mut roles = HashMap::new();
        {
            roles.insert("admin", "Administrator");
            roles.insert("root", "Root");
        }
        db.transaction::<_, Error, _>(move || {
            UserDao::sign_up(
                db,
                hmac,
                &self.real_name,
                &self.nick_name,
                &self.email,
                &self.password,
            )?;
            let user = UserDao::by_email(db, &self.email)?;
            LogDao::add(db, user.id, &ss.client_ip, "Init system.")?;

            ResouceDao::create(db, Resource::TYPE_ALL, Resource::CODE_ALL, "All in all")?;
            let resource =
                ResouceDao::by_type_and_code(db, Resource::TYPE_ALL, Resource::CODE_ALL)?;
            OperationDao::create(db, Operation::ALL, "All")?;
            let operation = OperationDao::by_code(db, Operation::ALL)?;
            let (nbf, exp) = PolicyItem::years(100)?;
            for (code, name) in roles.iter() {
                RoleDao::create(db, code, name, None)?;
                let role = RoleDao::by_code(db, code)?;
                PolicyDao::apply(db, role.id, resource.id, operation.id, &nbf, &exp)?;
                RoleDao::associate_user(db, role.id, user.id)?;
            }
            Ok(())
        })?;
        Ok(())
    }
    pub async fn execute(
        &self,
        ss: &Session,
        db: &Db,
        jwt: &Jwt,
        hmac: &Hmac,
        queue: &RabbitMq,
    ) -> Result<()> {
        self.validate()?;
        if UserDao::by_nick_name(db, &self.nick_name).is_ok() {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("Nick name already exists!".to_string()),
            )));
        }
        if UserDao::by_email(db, &self.email).is_ok() {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("Nick name already exists!".to_string()),
            )));
        }

        let user = db.transaction::<_, Error, _>(move || {
            UserDao::sign_up(
                db,
                hmac,
                &self.real_name,
                &self.nick_name,
                &self.email,
                &self.password,
            )?;
            let user = UserDao::by_email(db, &self.email)?;
            LogDao::add(db, user.id, &ss.client_ip, "Sign up.")?;
            Ok(user)
        })?;

        IdForm::send_email(&user, ss, db, jwt, queue, &self.home, Token::CONFIRM).await
    }
}

#[derive(Validate)]
pub struct SignInForm {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

fn by_nick_name_or_email(db: &Db, id: &str) -> Result<User> {
    let id = id.to_lowercase();
    if let Ok(it) = UserDao::by_nick_name(db, &id) {
        return Ok(it);
    }
    UserDao::by_email(db, &id)
}

#[derive(GraphQLObject)]
#[graphql(name = "UserSignInResponse")]
pub struct SignInResponse {
    pub token: String,
    pub user: CurrentUser,
}
impl SignInForm {
    pub fn execute(&self, ss: &Session, db: &Db, jwt: &Jwt, hmac: &Hmac) -> Result<SignInResponse> {
        self.validate()?;

        let user = by_nick_name_or_email(db, &self.id)?;

        if let Some(ref password) = user.password {
            if hmac.verify(password, self.password.as_bytes()) {
                user.available()?;
                // TODO expire time
                let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
                let token = jwt.sum(
                    None,
                    &Token {
                        uid: user.uid.clone(),
                        sub: user.real_name.clone(),
                        act: Token::SIGN_IN.to_string(),
                        nbf,
                        exp,
                    },
                )?;
                let uid = user.id;
                db.transaction::<_, Error, _>(move || {
                    LogDao::add(db, uid, &ss.client_ip, "Sign in successed.")?;
                    UserDao::sign_in(db, uid, &ss.client_ip)?;
                    Ok(())
                })?;
                let res = CurrentUser {
                    user: user.into(),
                    policies: PolicyDao::entities(db, uid)?,
                };
                return Ok(SignInResponse { token, user: res });
            }
        }

        LogDao::add(db, user.id, &ss.client_ip, "Sign in failed.")?;
        Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some("Incorrect password".to_string()),
        )))
    }
}

#[derive(Validate)]
pub struct IdForm {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub home: String,
}

impl IdForm {
    pub async fn confirm(&self, ss: &Session, db: &Db, jwt: &Jwt, queue: &RabbitMq) -> Result<()> {
        self.validate()?;
        let user = by_nick_name_or_email(db, &self.id)?;
        if user.confirmed_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("User already confirmed!".to_string()),
            )));
        }
        Self::send_email(&user, ss, db, jwt, queue, &self.home, Token::CONFIRM).await
    }
    pub async fn unlock(&self, ss: &Session, db: &Db, jwt: &Jwt, queue: &RabbitMq) -> Result<()> {
        self.validate()?;
        let user = by_nick_name_or_email(db, &self.id)?;
        if user.locked_at.is_none() {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("User isn't locked!".to_string()),
            )));
        }
        Self::send_email(&user, ss, db, jwt, queue, &self.home, Token::UNLOCK).await
    }
    pub async fn forgot_password(
        &self,
        ss: &Session,
        db: &Db,
        jwt: &Jwt,
        queue: &RabbitMq,
    ) -> Result<()> {
        self.validate()?;
        let user = by_nick_name_or_email(db, &self.id)?;
        user.available()?;
        Self::send_email(&user, ss, db, jwt, queue, &self.home, Token::RESET_PASSWORD).await
    }

    async fn send_email(
        user: &User,
        ss: &Session,
        db: &Db,
        jwt: &Jwt,
        queue: &RabbitMq,
        home: &str,
        act: &str,
    ) -> Result<()> {
        // TODO from settings table
        let ttl = Duration::hours(1);
        let (nbf, exp) = Jwt::timestamps(ttl);
        let token = jwt.sum(
            None,
            &Token {
                uid: user.uid.clone(),
                sub: user.real_name.clone(),
                act: act.to_string(),
                nbf,
                exp,
            },
        )?;
        let args = Some(serde_json::json!({
            "token": token,
            "name": user.real_name,
            "ttl": ttl.num_minutes(),
            "home": home,
        }));
        let subject = I18n::t(db, &ss.locale, format!("auth.user.{}.subject", act), &args);
        let body = I18n::t(db, &ss.locale, format!("auth.user.{}.body", act), &args);
        // TODO bcc from settings
        queue
            .publish(
                EmailTask::QUEUE,
                &EmailTask {
                    subject,
                    body,
                    media_type: MediaType::Html,
                    to: user.email.clone(),
                    cc: vec![],
                    bcc: vec![],
                    attachments: vec![],
                },
            )
            .await?;
        Ok(())
    }
}

#[derive(Validate)]
pub struct TokenForm {
    #[validate(length(min = 1))]
    pub token: String,
}

impl TokenForm {
    pub fn confirm(&self, ss: &Session, db: &Db, jwt: &Jwt) -> Result<()> {
        self.validate()?;
        let token: Token = jwt.parse(&self.token)?.claims;
        if token.act != Token::CONFIRM {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("Bad token!".to_string()),
            )));
        }
        db.transaction::<_, Error, _>(move || {
            let user = UserDao::by_uid(db, &token.uid)?;
            if user.confirmed_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("User already confirmed!".to_string()),
                )));
            }
            UserDao::confirm(db, user.id)?;
            LogDao::add(db, user.id, &ss.client_ip, "Confirmed.")?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn unlock(&self, ss: &Session, db: &Db, jwt: &Jwt) -> Result<()> {
        self.validate()?;
        let token: Token = jwt.parse(&self.token)?.claims;
        if token.act != Token::UNLOCK {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("Bad token!".to_string()),
            )));
        }
        db.transaction::<_, Error, _>(move || {
            let user = UserDao::by_uid(db, &token.uid)?;
            if user.locked_at.is_none() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("User isn't locked!".to_string()),
                )));
            }
            UserDao::lock(db, user.id, false)?;
            LogDao::add(db, user.id, &ss.client_ip, "Unlocked.")?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Validate)]
pub struct ResetPasswordForm {
    #[validate(length(min = 1))]
    pub token: String,
    #[validate(length(min = 6))]
    pub password: String,
}

impl ResetPasswordForm {
    pub fn execute(&self, ss: &Session, db: &Db, jwt: &Jwt, hmac: &Hmac) -> Result<()> {
        self.validate()?;

        let token: Token = jwt.parse(&self.token)?.claims;
        if token.act != Token::RESET_PASSWORD {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("Bad token!".to_string()),
            )));
        }
        db.transaction::<_, Error, _>(move || {
            let user = UserDao::by_uid(db, &token.uid)?;
            user.available()?;
            UserDao::set_password(db, hmac, user.id, &self.password)?;
            LogDao::add(db, user.id, &ss.client_ip, "Reset password.")?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Validate)]
pub struct ChangePasswordForm {
    #[validate(length(min = 1))]
    pub current_password: String,
    #[validate(length(min = 6))]
    pub new_password: String,
}

impl ChangePasswordForm {
    pub fn execute(&self, ss: &Session, db: &Db, jwt: &Jwt, hmac: &Hmac) -> Result<()> {
        self.validate()?;
        let user = ss.current_user(db, jwt)?;
        if let Some(ref password) = user.password {
            if hmac.verify(password, self.current_password.as_bytes()) {
                db.transaction::<_, Error, _>(move || {
                    UserDao::set_password(db, hmac, user.id, &self.current_password)?;
                    LogDao::add(db, user.id, &ss.client_ip, "Set password.")?;
                    Ok(())
                })?;
                return Ok(());
            }
        }
        Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some("Incorrect password".to_string()),
        )))
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "UserLogs")]
pub struct Logs {
    pub items: Vec<Log>,
    pub pagination: Pagination,
}

impl Logs {
    pub fn new(ss: &Session, page: &Page, db: &Db, jwt: &Jwt) -> Result<Self> {
        let user = ss.current_user(db, jwt)?;
        let total = LogDao::count(db, user.id)?;
        let (offset, limit) = page.pagination(total);
        let it = Self {
            items: LogDao::all(db, user.id, offset, limit)?,
            pagination: Pagination::new(total, page),
        };
        Ok(it)
    }
}

pub struct SignOut;

impl SignOut {
    pub fn execute(ss: &Session, db: &Db, jwt: &Jwt) -> Result<()> {
        let user = ss.current_user(db, jwt)?;
        LogDao::add(db, user.id, &ss.client_ip, "Sign out!")?;
        Ok(())
    }
}
