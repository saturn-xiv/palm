use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{
    google::oauth::openid::{
        AuthorizationCode as GoogleAuthorizationCode, IdToken as GoogleOpenIdToken,
    },
    orm::postgresql::Connection,
    Result,
};
use serde::{Deserialize, Serialize};

use super::super::super::schema::google_oauth2_users;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub subject: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
    pub code: Vec<u8>,
    pub token: Vec<u8>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn create(
        &mut self,
        user: i32,
        code: &GoogleAuthorizationCode,
        token: &GoogleOpenIdToken,
    ) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_subject(&mut self, subject: &str) -> Result<Item>;
    fn update(
        &mut self,
        id: i32,
        code: &GoogleAuthorizationCode,
        token: &GoogleOpenIdToken,
    ) -> Result<()>;
    fn total(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        user: i32,
        code: &GoogleAuthorizationCode,
        token: &GoogleOpenIdToken,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(google_oauth2_users::dsl::google_oauth2_users)
            .values((
                google_oauth2_users::dsl::user_id.eq(user),
                google_oauth2_users::dsl::subject.eq(&token.sub),
                google_oauth2_users::dsl::name.eq(&token.name),
                google_oauth2_users::dsl::email.eq(&token.email),
                google_oauth2_users::dsl::email_verified.eq(token.email_verified),
                google_oauth2_users::dsl::picture.eq(&token.picture),
                google_oauth2_users::dsl::locale.eq(&token.locale),
                google_oauth2_users::dsl::code.eq(&flexbuffers::to_vec(code)?),
                google_oauth2_users::dsl::token.eq(&flexbuffers::to_vec(token)?),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_subject(&mut self, subject: &str) -> Result<Item> {
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::subject.eq(subject))
            .first(self)?;
        Ok(it)
    }
    fn total(&mut self) -> Result<i64> {
        let it: i64 = google_oauth2_users::dsl::google_oauth2_users
            .count()
            .get_result(self)?;
        Ok(it)
    }
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = google_oauth2_users::dsl::google_oauth2_users
            .order(google_oauth2_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn update(
        &mut self,
        id: i32,
        code: &GoogleAuthorizationCode,
        token: &GoogleOpenIdToken,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                google_oauth2_users::dsl::subject.eq(&token.sub),
                google_oauth2_users::dsl::name.eq(&token.name),
                google_oauth2_users::dsl::email.eq(&token.email),
                google_oauth2_users::dsl::email_verified.eq(token.email_verified),
                google_oauth2_users::dsl::picture.eq(&token.picture),
                google_oauth2_users::dsl::locale.eq(&token.locale),
                google_oauth2_users::dsl::code.eq(&flexbuffers::to_vec(code)?),
                google_oauth2_users::dsl::token.eq(&flexbuffers::to_vec(token)?),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                google_oauth2_users::dsl::deleted_at.eq(&Some(now)),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = google_oauth2_users::dsl::google_oauth2_users
            .filter(google_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                google_oauth2_users::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                google_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
