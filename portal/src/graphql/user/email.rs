use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLInputObject;
use uuid::Uuid;
use validator::Validate;

use super::super::super::{
    Error, HttpError, Jwt, PasswordHashing, Result,
    cache::redis::StandaloneConnection as Cache,
    models::{
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
use super::SignInResponse;

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

        send_email(db, queue, jwt, &it, CONFIRM_ACTION).await?;
        Ok(())
    }
}

const CONFIRM_ACTION: &str = "user.confirm-by-email";

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

async fn send_email<J: Jwt>(
    _db: &mut Db,
    _queue: &RabbitMq,
    _jwt: &J,
    _user: &EmailUserItem,
    _action: &str,
) -> Result<()> {
    // TODO
    Ok(())
}
