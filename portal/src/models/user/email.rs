use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use hyacinth::schema::email_users;

use super::super::super::{PasswordHashing, Result, orm::postgresql::Connection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar: String,
    pub confirmed_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_email(&mut self, email: &str) -> Result<Item>;
    fn create<H: PasswordHashing>(
        &mut self,
        hash: &H,
        name: &str,
        email: &str,
        password: &str,
    ) -> impl Future<Output = Result<()>>;
    fn set_name(&mut self, id: i64, name: &str) -> Result<()>;
    fn set_password<H: PasswordHashing>(
        &mut self,
        hash: &H,
        id: i64,
        password: &str,
    ) -> impl Future<Output = Result<()>>;
    fn confirm(&mut self, id: i64) -> Result<()>;
    fn lock(&mut self, id: i64) -> Result<()>;
    fn unlock(&mut self, id: i64) -> Result<()>;
    fn delete(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn count(&mut self) -> Result<i64> {
        let it: i64 = email_users::dsl::email_users.count().get_result(self)?;
        Ok(it)
    }
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = email_users::dsl::email_users
            .order(email_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = email_users::dsl::email_users
            .filter(email_users::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_email(&mut self, email: &str) -> Result<Item> {
        let it = email_users::dsl::email_users
            .filter(email_users::dsl::email.eq(email))
            .first::<Item>(self)?;
        Ok(it)
    }
    async fn create<H: PasswordHashing>(
        &mut self,
        hash: &H,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<()> {
        let password = hash.sign(password).await?;
        let now = Utc::now().naive_utc();
        insert_into(email_users::dsl::email_users)
            .values((
                email_users::dsl::name.eq(name),
                email_users::dsl::email.eq(email),
                email_users::dsl::password.eq(&password),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_name(&mut self, id: i64, name: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::name.eq(name),
                email_users::dsl::version.eq(email_users::dsl::version + 1),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    async fn set_password<H: PasswordHashing>(
        &mut self,
        hash: &H,
        id: i64,
        password: &str,
    ) -> Result<()> {
        let password = hash.sign(password).await?;
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::password.eq(&password),
                email_users::dsl::version.eq(email_users::dsl::version + 1),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn confirm(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::confirmed_at.eq(&now),
                email_users::dsl::version.eq(email_users::dsl::version + 1),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn lock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::locked_at.eq(&now),
                email_users::dsl::version.eq(email_users::dsl::version + 1),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::locked_at.eq(None::<NaiveDateTime>),
                email_users::dsl::version.eq(email_users::dsl::version + 1),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set(email_users::dsl::deleted_at.eq(&now))
            .execute(self)?;
        Ok(())
    }
}
