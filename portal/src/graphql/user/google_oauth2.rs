use chrono::Duration;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLInputObject;
use uuid::Uuid;
use validator::Validate;

use super::super::super::{
    Dahlia, Error, HttpError, Jwt, Plugin, Result,
    cache::redis::StandaloneConnection as Cache,
    google::oauth2::scopes,
    models::{
        log::{Dao as LogDao, Level},
        user::{
            Dao as UserDao, Type as UserType, email::Dao as EmailUserDao,
            google_oauth2::Dao as GoogleOauth2UserDao,
        },
    },
    orm::postgresql::Connection as Db,
    queue::rabbitmq::Client as RabbitMq,
    random::alphanumeric as random_alphanumeric,
    rbac::Rbac,
};
use super::super::{CurrentUser, Session};
use super::{
    SignInResponse, State,
    email::{Confirm, send_email},
};

pub async fn user_for_sign_in(
    cache: &mut Cache,
    google: &Dahlia,
    ttl: Duration,
    redirect_uri: &str,
) -> Result<String> {
    authorize_url(cache, google, ttl, None, redirect_uri).await
}

pub async fn url_for_bind_user<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    cache: &mut Cache,
    jwt: &J,
    google: &Dahlia,
    ttl: Duration,
    redirect_uri: &str,
) -> Result<String> {
    let current_user = ss.current_user(db, cache, jwt).await?;
    authorize_url(cache, google, ttl, Some(&current_user), redirect_uri).await
}

async fn authorize_url(
    cache: &mut Cache,
    google: &Dahlia,
    ttl: Duration,
    user: Option<&CurrentUser>,
    redirect_uri: &str,
) -> Result<String> {
    let res = google
        .google_oauth2_sign_in_url(
            vec![scopes::EMAIL, scopes::PROFILE, scopes::OPENID],
            redirect_uri,
        )
        .await?;
    let tmp = State {
        user: user.map(|it| it.uid().clone()),
    };
    tmp.save(cache, res.state().to_str()?, ttl)?;
    Ok(res.authorization_url().to_string())
}

#[derive(Debug, Validate)]
pub struct BindEmail {
    #[validate(length(min = 5, max = 31), email)]
    pub email: String,
}
impl BindEmail {
    pub async fn execute<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        jwt: &J,
        queue: &RabbitMq,
    ) -> Result<()> {
        self.validate()?;
        let current_user = ss.current_user(db, cache, jwt).await?;
        if current_user.r#type != UserType::WechatOauth2 {
            return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
        }
        let ip = ss.client_ip();
        let password = random_alphanumeric(63);

        let it = db.transaction::<_, Error, _>(|tx| {
            EmailUserDao::create(
                tx,
                current_user.id(),
                current_user.name(),
                &self.email,
                &password,
            )?;
            let it = EmailUserDao::by_email(tx, &self.email)?;
            LogDao::create::<Plugin, _>(
                tx,
                it.user_id,
                Level::Info,
                ip,
                "Created by google oauth2.",
            )?;
            Ok(it)
        })?;
        send_email(db, queue, jwt, &it, Confirm::ACTION).await?;
        Ok(())
    }
}
