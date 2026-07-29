use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLInputObject;
use uuid::Uuid;
use validator::Validate;

use super::super::super::{
    Error, HttpError, Jwt, PasswordHashing, Rbac, Result,
    cache::redis::StandaloneConnection as Cache,
    models::{
        log::{Dao as LogDao, Level},
        user::{
            Dao as UserDao,
            email::{Dao as EmailUserDao, Item as EmailUserItem},
        },
    },
    orm::postgresql::Connection as Db,
    queue::rabbitmq::Client as RabbitMq,
};
use super::super::{Plugin, Session};

#[derive(Debug, Validate, GraphQLInputObject)]
#[graphql(name = "SetPasswordForEmailUser")]
pub struct SetPassword {
    #[validate(length(min = 1), email)]
    pub email: String,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
}

impl SetPassword {
    pub fn save(&self, db: &mut Db, password: &str) -> Result<()> {
        self.validate()?;
        let it = EmailUserDao::by_email(db, &self.email)?;
        EmailUserDao::set_password(db, it.id, password)?;

        Ok(())
    }

    pub async fn execute<R: Rbac, J: Jwt, H: PasswordHashing>(
        &self,
        db: &mut Db,
        cache: &mut Cache,
        jwt: &J,
        rbac: &R,
        ss: &Session,
        hashing: &H,
    ) -> Result<()> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        rbac.is_administrator(current_user.id).await?;

        let user_id = {
            let it = EmailUserDao::by_email(db, &self.email)?;
            if rbac.is_root(it.user_id).await.is_ok() {
                return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
            }
            it.user_id
        };
        let password = hashing.sign(&self.password).await?;
        let ip = ss.client_ip();

        db.transaction::<_, Error, _>(|tx| {
            self.save(tx, &password)?;

            LogDao::create::<Plugin, _>(
                tx,
                user_id,
                Level::Info,
                ip,
                format!("Reset password by administrator {}.", current_user.name),
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Debug, Validate, GraphQLInputObject)]
#[graphql(name = "UserSignUpByEmail")]
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
        let uid = Uuid::new_v4().to_string();

        UserDao::create(db, &uid, &self.lang.parse()?, self.timezone.parse()?)?;
        let user = UserDao::by_uid(db, &uid)?;
        EmailUserDao::create(db, user.id, &self.name, &self.email, password)?;
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

#[derive(Debug, Validate, GraphQLInputObject)]
#[graphql(name = "UserSignUpByEmail")]
pub struct SignIn {
    #[validate(length(min = 1, max = 31))]
    pub name: String,
    #[validate(length(min = 5, max = 31), email)]
    pub email: String,
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
