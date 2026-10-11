use std::fmt;
use std::result::Result as StdResult;

use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use diesel::{insert_into, prelude::*, result::Error as DieselError, update};
use hyacinth::{
    oauth2_v1::{GoogleUserInfoResponse as UserInfo, google_fetch_token_response::Credentials},
    schema::google_oauth2_users,
    to_option_str, to_vector_str,
};
use hyper::StatusCode;
use icu::locale::Locale;
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value, to_value};
use uuid::Uuid;

use super::super::super::{HttpError, Result, orm::postgresql::Connection};
use super::Dao as UserDao;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub uid: String,
    pub client_id: String,
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub hosted_domain: Option<String>,
    pub given_name: Option<String>,
    pub gender: Option<String>,
    pub link: Option<String>,
    pub family_name: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
    pub token: String,
    pub refresh_token: String,
    pub granted_scopes: Value,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn credentials(&self) -> Result<Credentials> {
        let mut it = Credentials::default();
        it.set_token(&self.token);
        it.set_refresh_token(&self.refresh_token);
        let granted_scopes: Vec<String> = from_value(self.granted_scopes.clone())?;
        for scope in granted_scopes {
            it.granted_scopes_mut().push(&scope);
        }
        Ok(it)
    }
    pub fn is_enable(&self) -> Result<()> {
        if !self.email_verified {
            return Err(Box::new(HttpError(
                StatusCode::NOT_ACCEPTABLE,
                Some("User's email address isn't verified yet".to_string()),
            )));
        }
        if self.locked_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::LOCKED,
                Some("User is locked".to_string()),
            )));
        }
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::GONE,
                Some("User is gone".to_string()),
            )));
        }
        Ok(())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}<{}>",
            self.name.as_deref().unwrap_or_default(),
            self.email
        )
    }
}

pub trait Dao {
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_user(&mut self, id: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn by_client_id_and_sub(&mut self, client_id: &str, sub: &str) -> StdResult<Item, DieselError>;
    fn by_client_id(&mut self, client_id: &str) -> Result<Vec<Item>>;
    fn create(
        &mut self,
        user: i64,
        client_id: &str,
        credentials: &Credentials,
        user_info: &UserInfo,
    ) -> Result<()>;
    fn set_info(&mut self, id: i64, user_info: &UserInfo) -> Result<()>;
    fn set_credentials(&mut self, id: i64, credentials: &Credentials) -> Result<()>;
    fn lock(&mut self, id: i64) -> Result<()>;
    fn unlock(&mut self, id: i64) -> Result<()>;
    fn delete(&mut self, id: i64) -> Result<()>;
    fn sign_in_or_up(
        &mut self,
        location: (&Locale, Tz, Option<&str>),
        client_id: &str,
        credentials: &Credentials,
        user_info: &UserInfo,
    ) -> Result<Item>;
}

impl Dao for Connection {
    fn count(&mut self) -> Result<i64> {
        let it: i64 = google_oauth2_users::dsl::google_oauth2_users
            .count()
            .get_result(self)?;
        Ok(it)
    }
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = google_oauth2_users::dsl::google_oauth2_users
            .order(google_oauth2_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_user(&mut self, id: i64) -> Result<Vec<Item>> {
        let it = google_oauth2_users::dsl::google_oauth2_users
            .order(google_oauth2_users::dsl::updated_at.desc())
            .filter(google_oauth2_users::dsl::user_id.eq(id))
            .load::<Item>(self)?;
        Ok(it)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::uid.eq(uid))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_client_id_and_sub(&mut self, client_id: &str, sub: &str) -> StdResult<Item, DieselError> {
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::client_id.eq(client_id))
            .filter(google_oauth2_users::dsl::sub.eq(sub))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_client_id(&mut self, client_id: &str) -> Result<Vec<Item>> {
        let it = google_oauth2_users::dsl::google_oauth2_users
            .order(google_oauth2_users::dsl::updated_at.desc())
            .filter(google_oauth2_users::dsl::client_id.eq(client_id))
            .load::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        client_id: &str,
        credentials: &Credentials,
        user_info: &UserInfo,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let uid = Uuid::new_v4().to_string();
        let granted_scopes = to_value(to_vector_str(credentials.granted_scopes())?)?;

        insert_into(google_oauth2_users::dsl::google_oauth2_users)
            .values((
                google_oauth2_users::dsl::uid.eq(&uid),
                google_oauth2_users::dsl::user_id.eq(user),
                google_oauth2_users::dsl::client_id.eq(client_id),
                google_oauth2_users::dsl::sub.eq(&user_info.sub().to_str()?),
                google_oauth2_users::dsl::token.eq(&credentials.token().to_str()?),
                google_oauth2_users::dsl::refresh_token
                    .eq(&credentials.refresh_token().to_str()?),
                google_oauth2_users::dsl::granted_scopes.eq(&granted_scopes),
                google_oauth2_users::dsl::email.eq(&user_info.email().to_str()?),
                google_oauth2_users::dsl::email_verified.eq(user_info.email_verified()),
                google_oauth2_users::dsl::hosted_domain
                    .eq(to_option_str(user_info.hosted_domain_opt())?),
                google_oauth2_users::dsl::given_name.eq(to_option_str(user_info.given_name_opt())?),
                google_oauth2_users::dsl::gender.eq(to_option_str(user_info.gender_opt())?),
                google_oauth2_users::dsl::link.eq(to_option_str(user_info.link_opt())?),
                google_oauth2_users::dsl::family_name
                    .eq(to_option_str(user_info.family_name_opt())?),
                google_oauth2_users::dsl::name.eq(to_option_str(user_info.name_opt())?),
                google_oauth2_users::dsl::picture.eq(to_option_str(user_info.picture_opt())?),
                google_oauth2_users::dsl::locale.eq(to_option_str(user_info.locale_opt())?),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_info(&mut self, id: i64, user_info: &UserInfo) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                google_oauth2_users::dsl::email.eq(&user_info.email().to_str()?),
                google_oauth2_users::dsl::email_verified.eq(&user_info.email_verified()),
                google_oauth2_users::dsl::hosted_domain
                    .eq(to_option_str(user_info.hosted_domain_opt())?),
                google_oauth2_users::dsl::given_name.eq(to_option_str(user_info.given_name_opt())?),
                google_oauth2_users::dsl::gender.eq(to_option_str(user_info.gender_opt())?),
                google_oauth2_users::dsl::link.eq(to_option_str(user_info.link_opt())?),
                google_oauth2_users::dsl::family_name
                    .eq(to_option_str(user_info.family_name_opt())?),
                google_oauth2_users::dsl::name.eq(to_option_str(user_info.name_opt())?),
                google_oauth2_users::dsl::picture.eq(to_option_str(user_info.picture_opt())?),
                google_oauth2_users::dsl::locale.eq(to_option_str(user_info.locale_opt())?),
                google_oauth2_users::dsl::version.eq(google_oauth2_users::dsl::version + 1),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_credentials(&mut self, id: i64, credentials: &Credentials) -> Result<()> {
        let now = Utc::now().naive_utc();
        let granted_scopes = to_value(to_vector_str(credentials.granted_scopes())?)?;
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                google_oauth2_users::dsl::token.eq(&credentials.token().to_str()?),
                google_oauth2_users::dsl::refresh_token
                    .eq(&credentials.refresh_token().to_str()?),
                google_oauth2_users::dsl::granted_scopes.eq(&granted_scopes),
                google_oauth2_users::dsl::version.eq(google_oauth2_users::dsl::version + 1),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn lock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                google_oauth2_users::dsl::locked_at.eq(&now),
                google_oauth2_users::dsl::version.eq(google_oauth2_users::dsl::version + 1),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                google_oauth2_users::dsl::locked_at.eq(None::<NaiveDateTime>),
                google_oauth2_users::dsl::version.eq(google_oauth2_users::dsl::version + 1),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id));
        update(it)
            .set(google_oauth2_users::dsl::deleted_at.eq(&now))
            .execute(self)?;
        Ok(())
    }

    fn sign_in_or_up(
        &mut self,
        (lang, timezone, state): (&Locale, Tz, Option<&str>),
        client_id: &str,
        credentials: &Credentials,
        user_info: &UserInfo,
    ) -> Result<Item> {
        let sub = user_info.sub().to_str()?;
        match self.by_client_id_and_sub(client_id, sub) {
            Ok(it) => {
                it.is_enable()?;
                {
                    let user = UserDao::by_id(self, it.user_id)?;
                    user.is_enable()?;
                }
                self.set_credentials(it.id, credentials)?;
                self.set_info(it.id, user_info)?;
                Ok(())
            }
            Err(DieselError::NotFound) => {
                let user = match state {
                    Some(uid) => {
                        let it = UserDao::by_uid(self, uid)?;
                        it.is_enable()?;
                        it
                    }
                    None => {
                        let uid = Uuid::new_v4().to_string();
                        UserDao::create(self, &uid, "Google Oauth2 User", lang, timezone)?;
                        UserDao::by_uid(self, &uid)?
                    }
                };
                Dao::create(self, user.id, client_id, credentials, user_info)?;
                Ok(())
            }
            Err(e) => Err(e),
        }?;
        Ok(self.by_client_id_and_sub(client_id, sub)?)
    }
}
