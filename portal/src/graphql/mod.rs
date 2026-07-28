pub mod currency;
pub mod locale;
pub mod user;

use axum::http::HeaderMap;
use chrono::{NaiveDateTime, Utc};
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

use super::{
    HttpError, Jwt, Result,
    cache::redis::StandaloneClient as Cache,
    headers::{AUTHORIZATION, BEARER, X_FORWARDED_FOR, X_REAL_IP},
    models::user::{Dao as UserDao, Type as UserType, email::Dao as EmailUserDao},
    orm::postgresql::Connection as Db,
};

pub const QUEUE_SMS_BY_TWILIO: &str = "sms-send.twilio";
pub const QUEUE_TEX: &str = "tex";
pub const QUEUE_EMAIL_SEND: &str = "email-send";
pub const QUEUE_CUPS: &str = "cups";

pub struct Session {
    pub client_ip: String,
    pub token: Option<String>,
}

impl Session {
    pub fn new(headers: &HeaderMap) -> Self {
        Self {
            token: Self::token(headers),
            client_ip: Self::client_ip(headers).unwrap_or_default(),
        }
    }

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

    fn token(headers: &HeaderMap) -> Option<String> {
        if let Some(auth) = headers.get(AUTHORIZATION)
            && let Ok(auth) = auth.to_str()
            && let Some(token) = auth.strip_prefix(BEARER)
        {
            return Some(token.to_string());
        }
        None
    }

    /*
    nginx.conf

    location / {
        proxy_pass http://localhost:8080;

        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header Host $http_host;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
     */
    fn client_ip(headers: &HeaderMap) -> Option<String> {
        if let Some(it) = headers.get(X_FORWARDED_FOR)
            && let Ok(it) = it.to_str()
        {
            for it in it.split(",") {
                let it = it.trim();
                if !it.is_empty() {
                    return Some(it.to_string());
                }
            }
        }
        if let Some(it) = headers.get(X_REAL_IP)
            && let Ok(it) = it.to_str()
        {
            let it = it.trim();
            if !it.is_empty() {
                return Some(it.to_string());
            }
        }
        None
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

#[derive(Debug, GraphQLObject)]
#[graphql(name = "Succeeded")]
pub struct Succeeded {
    pub created_at: NaiveDateTime,
}
impl Default for Succeeded {
    fn default() -> Self {
        Self {
            created_at: Utc::now().naive_utc(),
        }
    }
}
