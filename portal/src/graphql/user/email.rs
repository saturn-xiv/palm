use diesel::Connection as DieselConnection;
use juniper::GraphQLInputObject;
use uuid::Uuid;
use validator::Validate;

use super::super::super::{
    Error, PasswordHashing, Result,
    models::{
        log::{Dao as LogDao, Level},
        user::{
            Dao as UserDao,
            email::{Dao as EmailUserDao, Item as EmailUserItem},
        },
    },
    orm::postgresql::Connection as Db,
    queue::rabbitmq::Node as RabbitMq,
};
use super::super::{Plugin, Session};

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
    pub fn create(&self, db: &mut Db, password: &str) -> Result<String> {
        self.validate()?;
        let uid = Uuid::new_v4().to_string();
        let email = self.email.trim().to_lowercase();
        UserDao::create(db, &uid, &self.lang.parse()?, self.timezone.parse()?)?;
        let user = UserDao::by_uid(db, &uid)?;
        EmailUserDao::create(db, user.id, &self.name, &email, password)?;
        Ok(email)
    }

    pub async fn execute<H: PasswordHashing>(
        &self,
        db: &mut Db,
        queue: &RabbitMq,
        ss: &Session,
        hashing: &H,
    ) -> Result<()> {
        let password = hashing.sign(&self.password).await?;

        let it = db.transaction::<_, Error, _>(|tx| {
            let email = self.create(tx, &password)?;
            let it = EmailUserDao::by_email(tx, &email)?;
            LogDao::create::<Plugin, _>(tx, it.user_id, Level::Info, &ss.client_ip, "Sign up.")?;
            Ok(it)
        })?;

        send_email(db, queue, &it, CONFIRM_ACTION).await?;
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

async fn send_email(
    _db: &mut Db,
    _queue: &RabbitMq,
    _user: &EmailUserItem,
    _action: &str,
) -> Result<()> {
    // TODO
    Ok(())
}
