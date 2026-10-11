use chrono::Duration;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLInputObject;
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

#[derive(Clone, Debug, Validate, GraphQLInputObject)]
#[graphql(name = "GoogleOauth2UserCallbackRequest")]
pub struct Callback {
    #[validate(url, length(min = 1, max = 255))]
    pub authorization_response: String,
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
        (rbac, jwt, google): (&R, &J, &Dahlia),
        version: &str,
    ) -> Result<SignInResponse> {
        self.validate()?;

        let token = google
            .google_oauth2_fetch_token(
                &self.authorization_response,
                vec![scopes::EMAIL, scopes::PROFILE, scopes::OPENID],
                &self.state,
            )
            .await?;
        let client_id = token.client_id().to_str()?;
        let credentials = token.credentials().to_owned();
        let user_info = google.google_oauth2_user_info(credentials.clone()).await?;
        let state = State::new(cache, &self.state)?;

        let ip = ss.client_ip();

        let it = db.transaction::<_, Error, _>(|tx| {
            let it = GoogleOauth2UserDao::sign_in_or_up(
                tx,
                (
                    &self.lang.parse()?,
                    self.timezone.parse()?,
                    state.user.as_deref(),
                ),
                client_id,
                &credentials,
                &user_info,
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
