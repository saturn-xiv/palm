use std::string::ToString;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use juniper::GraphQLObject;
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::bookkeeper_logs;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub user_id: i32,
    pub action: String,
    pub detail: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(GraphQLObject, Serialize, Deserialize, Default, Clone)]
#[graphql(name = "BookkeeperLogDetail")]
pub struct Detail {
    pub user: String,
    pub memo: String,
    pub reason: Option<String>,
}

impl Detail {
    pub fn new(buf: &[u8]) -> Result<Self> {
        let it = flexbuffers::from_slice(buf)?;
        Ok(it)
    }
}

pub trait Dao {
    fn create(&mut self, ledger: i32, user: i32, action: &str, detail: &Detail) -> Result<String>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn count_by_ledger(&mut self, ledger: i32) -> Result<i64>;
    fn by_ledger(&mut self, ledger: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn create(&mut self, ledger: i32, user: i32, action: &str, detail: &Detail) -> Result<String> {
        let uid = Uuid::new_v4().to_string();
        let detail = flexbuffers::to_vec(detail)?;
        insert_into(bookkeeper_logs::dsl::bookkeeper_logs)
            .values((
                bookkeeper_logs::dsl::ledger_id.eq(ledger),
                bookkeeper_logs::dsl::user_id.eq(&user),
                bookkeeper_logs::dsl::action.eq(action),
                bookkeeper_logs::dsl::detail.eq(&detail),
            ))
            .execute(self)?;
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
