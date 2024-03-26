pub mod history;

use camelia::orm::postgresql::Connection;
use chrono::{NaiveDateTime, Utc};
use diesel::{data_types::Cents, delete, insert_into, prelude::*, update};
use palm::Result;

use super::super::schema::daffodil_bills;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub ledger_id: i32,
    pub summary: String,
    pub amount: Cents,
    pub currency: String,
    pub merchant: String,
    pub category: String,
    pub paid_at: NaiveDateTime,
    pub paid_by: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = daffodil_bills)]
pub struct New<'a> {
    pub user_id: i32,
    pub ledger_id: i32,
    pub summary: &'a str,
    pub amount: Cents,
    pub currency: &'a str,
    pub merchant: &'a str,
    pub category: &'a str,
    pub paid_at: &'a NaiveDateTime,
    pub paid_by: &'a str,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>>;
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>>;
    fn create(
        &mut self,
        user: i32,
        ledger: i32,
        summary: &str,
        amount: (i64, &str),
        category: &str,
        paid: (&str, &NaiveDateTime, &str),
    ) -> Result<()>;
    fn update(
        &mut self,
        id: i32,
        user: i32,
        summary: &str,
        amount: (i64, &str),
        category: &str,
        paid: (&str, &NaiveDateTime, &str),
    ) -> Result<()>;
    fn delete(&mut self, id: i32) -> Result<()>;

    fn merchants(&mut self, user: i32) -> Result<Vec<String>>;
    fn categories(&mut self, user: i32) -> Result<Vec<String>>;
    fn payment_methods(&mut self, user: i32) -> Result<Vec<String>>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .filter(daffodil_bills::dsl::id.eq(id))
            .first::<Item>(self)?)
    }
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .filter(daffodil_bills::dsl::user_id.eq(user))
            .order(daffodil_bills::dsl::paid_at.desc())
            .load::<Item>(self)?)
    }
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .filter(daffodil_bills::dsl::ledger_id.eq(ledger))
            .order(daffodil_bills::dsl::paid_at.desc())
            .load::<Item>(self)?)
    }
    fn create(
        &mut self,
        user: i32,
        ledger: i32,
        summary: &str,
        (amount, currency): (i64, &str),
        category: &str,
        (merchant, paid_at, paid_by): (&str, &NaiveDateTime, &str),
    ) -> Result<()> {
        insert_into(daffodil_bills::dsl::daffodil_bills)
            .values(&New {
                user_id: user,
                ledger_id: ledger,
                summary,
                amount: Cents(amount),
                merchant,
                currency,
                category,
                paid_at,
                paid_by,
                updated_at: &Utc::now().naive_utc(),
            })
            .execute(self)?;
        Ok(())
    }
    fn update(
        &mut self,
        id: i32,
        user: i32,
        summary: &str,
        (amount, currency): (i64, &str),
        category: &str,
        (merchant, paid_at, paid_by): (&str, &NaiveDateTime, &str),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(daffodil_bills::dsl::daffodil_bills)
            .filter(daffodil_bills::dsl::id.eq(id))
            .set((
                daffodil_bills::dsl::user_id.eq(user),
                daffodil_bills::dsl::summary.eq(summary),
                daffodil_bills::dsl::amount.eq(Cents(amount)),
                daffodil_bills::dsl::currency.eq(currency),
                daffodil_bills::dsl::category.eq(category),
                daffodil_bills::dsl::merchant.eq(merchant),
                daffodil_bills::dsl::paid_at.eq(paid_at),
                daffodil_bills::dsl::paid_by.eq(paid_by),
                daffodil_bills::dsl::version.eq(daffodil_bills::dsl::version + 1),
                daffodil_bills::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&mut self, id: i32) -> Result<()> {
        delete(daffodil_bills::dsl::daffodil_bills.filter(daffodil_bills::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }

    fn merchants(&mut self, user: i32) -> Result<Vec<String>> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .select(daffodil_bills::dsl::merchant)
            .filter(daffodil_bills::dsl::user_id.eq(user))
            .distinct()
            .load::<String>(self)?)
    }
    fn categories(&mut self, user: i32) -> Result<Vec<String>> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .select(daffodil_bills::dsl::category)
            .filter(daffodil_bills::dsl::user_id.eq(user))
            .distinct()
            .load::<String>(self)?)
    }
    fn payment_methods(&mut self, user: i32) -> Result<Vec<String>> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .select(daffodil_bills::dsl::paid_by)
            .filter(daffodil_bills::dsl::user_id.eq(user))
            .distinct()
            .load::<String>(self)?)
    }
}
