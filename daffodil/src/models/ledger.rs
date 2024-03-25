use camelia::orm::postgresql::Connection;
use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use palm::Result;
use uuid::Uuid;

use super::super::schema::daffodil_ledgers;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub owner_id: i32,
    pub uid: String,
    pub name: String,
    pub summary: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub const SHOW: &'static str = "ledger-show";
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn by_user_and_name(&mut self, user: i32, name: &str) -> Result<Item>;
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>>;
    fn create(&mut self, user: i32, name: &str, summary: &str) -> Result<()>;
    fn update(&mut self, id: i32, name: &str, summary: &str) -> Result<()>;
    fn enable(&mut self, id: i32, on: bool) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        Ok(daffodil_ledgers::dsl::daffodil_ledgers
            .filter(daffodil_ledgers::dsl::id.eq(id))
            .first::<Item>(self)?)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        Ok(daffodil_ledgers::dsl::daffodil_ledgers
            .filter(daffodil_ledgers::dsl::uid.eq(uid))
            .first::<Item>(self)?)
    }
    fn by_user_and_name(&mut self, user: i32, name: &str) -> Result<Item> {
        Ok(daffodil_ledgers::dsl::daffodil_ledgers
            .filter(daffodil_ledgers::dsl::owner_id.eq(user))
            .filter(daffodil_ledgers::dsl::name.eq(name))
            .first::<Item>(self)?)
    }
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>> {
        Ok(daffodil_ledgers::dsl::daffodil_ledgers
            .filter(daffodil_ledgers::dsl::owner_id.eq(user))
            .order(daffodil_ledgers::dsl::updated_at.desc())
            .load::<Item>(self)?)
    }
    fn create(&mut self, user: i32, name: &str, summary: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let uid = Uuid::new_v4().to_string();
        insert_into(daffodil_ledgers::dsl::daffodil_ledgers)
            .values((
                daffodil_ledgers::dsl::owner_id.eq(user),
                daffodil_ledgers::dsl::uid.eq(&uid),
                daffodil_ledgers::dsl::name.eq(name),
                daffodil_ledgers::dsl::summary.eq(summary),
                daffodil_ledgers::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, name: &str, summary: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(daffodil_ledgers::dsl::daffodil_ledgers)
            .filter(daffodil_ledgers::dsl::id.eq(id))
            .set((
                daffodil_ledgers::name.eq(name),
                daffodil_ledgers::dsl::summary.eq(summary),
                daffodil_ledgers::dsl::updated_at.eq(&now),
                daffodil_ledgers::dsl::version.eq(daffodil_ledgers::dsl::version + 1),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();

        update(daffodil_ledgers::dsl::daffodil_ledgers)
            .filter(daffodil_ledgers::dsl::id.eq(id))
            .set((
                daffodil_ledgers::dsl::deleted_at.eq(&if on { None } else { Some(now) }),
                daffodil_ledgers::dsl::version.eq(daffodil_ledgers::dsl::version + 1),
                daffodil_ledgers::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;

        Ok(())
    }
}
