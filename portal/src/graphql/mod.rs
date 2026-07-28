pub mod user;

use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

use super::{
    HttpError, Jwt, Result,
    cache::redis::StandaloneClient as Cache,
    models::user::{Dao as UserDao, Type as UserType, email::Dao as EmailUserDao},
    orm::postgresql::Connection as Db,
};

pub const QUEUE_SMS_BY_TWILIO: &str = "sms-send.twilio";
pub const QUEUE_TEX: &str = "tex";
pub const QUEUE_EMAIL_SEND: &str = "email-send";
pub const QUEUE_CUPS: &str = "cups";

pub struct Session {
    pub client_ip: String,
    token: Option<String>,
}

impl Session {
    pub async fn current_user<J: Jwt>(
        &self,
        db: &mut Db,
        cache: &mut Cache,
        jwt: &J,
    ) -> Result<CurrentUser> {
        let token = self
            .token
            .as_ref()
            .ok_or_else(|| Box::new(HttpError(StatusCode::NON_AUTHORITATIVE_INFORMATION, None)))?;

        CurrentUser::new(db, cache, jwt, token).await
    }
}

pub struct CurrentUser {
    pub id: i64,
    pub name: String,
}
impl CurrentUser {
    pub async fn new<J: Jwt>(
        db: &mut Db,
        _cache: &mut Cache,
        jwt: &J,
        token: &str,
    ) -> Result<CurrentUser> {
        let (subject, payload) = jwt
            .verify::<TokenPayload>(token, Self::ISSUER, Self::SIGN_IN_AUDIENCE)
            .await?;
        let payload = payload.ok_or_else(|| {
            Box::new(HttpError(
                StatusCode::FORBIDDEN,
                Some("Invalid token".to_string()),
            ))
        })?;

        let it = match payload.r#type {
            UserType::Email => {
                let it = EmailUserDao::by_email(db, &subject)?;
                if it.locked_at.is_some() {
                    Err(Box::new(HttpError(
                        StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        Some("User isn't confirmed yet".to_string()),
                    )))
                } else if it.deleted_at.is_some() {
                    Err(Box::new(HttpError(
                        StatusCode::GONE,
                        Some("User is locked".to_string()),
                    )))
                } else if it.confirmed_at.is_none() {
                    Err(Box::new(HttpError(
                        StatusCode::FORBIDDEN,
                        Some("User is locked".to_string()),
                    )))
                } else {
                    Ok(Self {
                        id: it.user_id,
                        name: it.name,
                    })
                }
            }
            _ => Err(Box::new(HttpError(
                StatusCode::FORBIDDEN,
                Some("Invalid user type".to_string()),
            ))),
        }?;

        {
            let user = UserDao::by_id(db, it.id)?;
            if user.locked_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::LOCKED,
                    Some("User is locked".to_string()),
                )));
            }
            if user.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("User is locked".to_string()),
                )));
            }
        }
        Ok(it)
    }

    pub const ISSUER: &str = "Palm";
    pub const SIGN_IN_AUDIENCE: &str = "user.sign-in";
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenPayload {
    r#type: UserType,
}

pub struct Plugin;

#[derive(Debug, GraphQLObject)]
#[graphql(name = "Pagination")]
pub struct Pagination {
    pub page: i32,
    pub size: i32,
    pub total: i32,
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(name = "Page")]
pub struct Page {
    pub index: i32,
    pub size: i32,
}
