use std::string::ToString;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use uuid::Uuid;

use super::super::schema::bookkeeper_logs;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub user_id: i32,
    pub username: String,
    pub action: String,
    pub memo: String,
    pub reason: Option<String>,
    pub ip: String,
    pub created_at: NaiveDateTime,
}

#[derive(
    EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone, Copy,
)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    #[default]
    CreateTransaction,
    UpdateLedge,
    CreateLedge,
}

pub trait Dao {
    fn create(
        &mut self,
        ledger: i32,
        user: (i32, &str),
        details: (Action, &str, Option<&str>),
        ip: &str,
    ) -> Result<String>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn count_by_ledger(&mut self, ledger: i32) -> Result<i64>;
    fn by_ledger(&mut self, ledger: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        ledger: i32,
        (user_id, username): (i32, &str),
        (action, memo, reason): (Action, &str, Option<&str>),
        ip: &str,
    ) -> Result<String> {
        let uid = Uuid::new_v4().to_string();

        match reason {
            Some(reason) => {
                insert_into(bookkeeper_logs::dsl::bookkeeper_logs)
                    .values((
                        bookkeeper_logs::dsl::ledger_id.eq(ledger),
                        bookkeeper_logs::dsl::user_id.eq(user_id),
                        bookkeeper_logs::dsl::username.eq(username),
                        bookkeeper_logs::dsl::action.eq(&action.to_string()),
                        bookkeeper_logs::dsl::memo.eq(&memo),
                        bookkeeper_logs::dsl::reason.eq(&reason),
                        bookkeeper_logs::dsl::ip.eq(ip),
                    ))
                    .execute(self)?;
            }
            None => {
                insert_into(bookkeeper_logs::dsl::bookkeeper_logs)
                    .values((
                        bookkeeper_logs::dsl::ledger_id.eq(ledger),
                        bookkeeper_logs::dsl::user_id.eq(user_id),
                        bookkeeper_logs::dsl::username.eq(username),
                        bookkeeper_logs::dsl::action.eq(&action.to_string()),
                        bookkeeper_logs::dsl::memo.eq(&memo),
                        bookkeeper_logs::dsl::ip.eq(ip),
                    ))
                    .execute(self)?;
            }
        }

        Ok(uid)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bookkeeper_logs::dsl::bookkeeper_logs
            .filter(bookkeeper_logs::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn count_by_ledger(&mut self, ledger: i32) -> Result<i64> {
        let cnt: i64 = bookkeeper_logs::dsl::bookkeeper_logs
            .count()
            .filter(bookkeeper_logs::dsl::ledger_id.eq(ledger))
            .get_result(self)?;
        Ok(cnt)
    }
    fn by_ledger(&mut self, ledger: i32, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = bookkeeper_logs::dsl::bookkeeper_logs
            .filter(bookkeeper_logs::dsl::ledger_id.eq(ledger))
            .order(bookkeeper_logs::dsl::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
}
