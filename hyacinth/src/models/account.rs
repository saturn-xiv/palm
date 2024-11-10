use std::string::ToString;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use juniper::GraphQLEnum;
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::bookkeeper_accounts;

// Assets - Liabilities = Equity
// Assets - Liabilities = Equity + (Income - Expenses)
// Assets + Expenses = Liabilities + Income + Equity
#[derive(
    GraphQLEnum,
    EnumDisplay,
    EnumString,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
)]
#[serde(rename_all = "camelCase")]
#[graphql(name = "BookkeeperAccountType")]
pub enum Type {
    #[default]
    Cash,
    Bank,
    Stock,
    MutualFund,
    AccountsReceivable,
    OtherAssets,

    CreditCard,
    AccountsPayable,
    Liability,

    Equity,

    Income,

    Expenses,
}

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub parent_id: Option<i32>,
    pub label: String,
    pub memo: String,
    pub currency_id: i32,
    pub r#type: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn create(
        &mut self,
        ledger: i32,
        parent: Option<i32>,
        label: &str,
        memo: &str,
        type_: Type,
        currency: i32,
    ) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>>;
    fn by_parent(&mut self, parent: i32) -> Result<Vec<Item>>;
    fn set_details(&mut self, id: i32, label: &str, memo: &str) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        ledger: i32,
        parent: Option<i32>,
        label: &str,
        memo: &str,
        type_: Type,
        currency: i32,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(bookkeeper_accounts::dsl::bookkeeper_accounts)
            .values((
                bookkeeper_accounts::dsl::ledger_id.eq(ledger),
                bookkeeper_accounts::dsl::parent_id.eq(parent),
                bookkeeper_accounts::dsl::label.eq(label),
                bookkeeper_accounts::dsl::memo.eq(memo),
                bookkeeper_accounts::dsl::currency_id.eq(currency),
                bookkeeper_accounts::dsl::type_.eq(&type_.to_string()),
                bookkeeper_accounts::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bookkeeper_accounts::dsl::bookkeeper_accounts
            .filter(bookkeeper_accounts::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_accounts::dsl::bookkeeper_accounts
            .filter(bookkeeper_accounts::dsl::ledger_id.eq(ledger))
            .order(bookkeeper_accounts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_parent(&mut self, parent: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_accounts::dsl::bookkeeper_accounts
            .filter(bookkeeper_accounts::dsl::parent_id.eq(parent))
            .order(bookkeeper_accounts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn set_details(&mut self, id: i32, label: &str, memo: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_accounts::dsl::bookkeeper_accounts
            .filter(bookkeeper_accounts::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_accounts::dsl::label.eq(label),
                bookkeeper_accounts::dsl::memo.eq(memo),
                bookkeeper_accounts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_accounts::dsl::bookkeeper_accounts
            .filter(bookkeeper_accounts::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_accounts::dsl::deleted_at.eq(&Some(now)),))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let it = bookkeeper_accounts::dsl::bookkeeper_accounts
            .filter(bookkeeper_accounts::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_accounts::dsl::deleted_at.eq(&None::<NaiveDateTime>),))
            .execute(self)?;
        Ok(())
    }
}
