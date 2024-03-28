pub mod history;

use camelia::orm::postgresql::Connection;
use chrono::{NaiveDateTime, Utc};
use diesel::{delete, dsl::sum, insert_into, prelude::*, update};
use palm::Result;

use super::super::schema::daffodil_bills;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub ledger_id: i32,
    pub summary: String,
    pub amount: i32,
    pub currency: String,
    pub merchant: String,
    pub category: String,
    pub paid_at: NaiveDateTime,
    pub paid_by: String,
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
    pub amount: i32,
    pub currency: &'a str,
    pub merchant: &'a str,
    pub category: &'a str,
    pub paid_at: &'a NaiveDateTime,
    pub paid_by: &'a str,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn first_by_ledger(&mut self, ledger: i32) -> Result<Item>;
    fn latest_by_ledger(&mut self, ledger: i32) -> Result<Item>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>>;
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>>;
    fn by_currency_ledger_and_dates(
        &mut self,
        ledger: i32,
        currency: &str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<Vec<Item>>;
    fn inventory_by_currency_ledger_and_dates(
        &mut self,
        ledger: i32,
        currency: &str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<(i64, i64, i64)>;
    fn create(
        &mut self,
        user: i32,
        ledger: i32,
        summary: &str,
        amount: (i32, &str),
        category: &str,
        paid: (&str, &NaiveDateTime, &str),
    ) -> Result<()>;
    fn update(
        &mut self,
        id: i32,
        user: i32,
        summary: &str,
        amount: (i32, &str),
        category: &str,
        paid: (&str, &NaiveDateTime, &str),
    ) -> Result<()>;
    fn delete(&mut self, id: i32) -> Result<()>;
    fn merchants(&mut self, user: i32) -> Result<Vec<String>>;
    fn currencies(&mut self, ledger: i32) -> Result<Vec<String>>;
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
    fn first_by_ledger(&mut self, ledger: i32) -> Result<Item> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .filter(daffodil_bills::dsl::ledger_id.eq(ledger))
            .order(daffodil_bills::dsl::paid_at.asc())
            .first::<Item>(self)?)
    }
    fn latest_by_ledger(&mut self, ledger: i32) -> Result<Item> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .filter(daffodil_bills::dsl::ledger_id.eq(ledger))
            .order(daffodil_bills::dsl::paid_at.desc())
            .first::<Item>(self)?)
    }
    fn by_currency_ledger_and_dates(
        &mut self,
        ledger: i32,
        currency: &str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<Vec<Item>> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .filter(daffodil_bills::dsl::ledger_id.eq(ledger))
            .filter(daffodil_bills::dsl::currency.eq(currency))
            .filter(daffodil_bills::dsl::paid_at.ge(from))
            .filter(daffodil_bills::dsl::paid_at.lt(to))
            .order(daffodil_bills::dsl::paid_at.desc())
            .load::<Item>(self)?)
    }
    fn inventory_by_currency_ledger_and_dates(
        &mut self,
        ledger: i32,
        currency: &str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<(i64, i64, i64)> {
        let income: Option<i64> = daffodil_bills::dsl::daffodil_bills
            .select(sum(daffodil_bills::dsl::version))
            .filter(daffodil_bills::dsl::ledger_id.eq(ledger))
            .filter(daffodil_bills::dsl::currency.eq(currency))
            .filter(daffodil_bills::dsl::paid_at.ge(from))
            .filter(daffodil_bills::dsl::paid_at.lt(to))
            .filter(daffodil_bills::dsl::amount.gt(0))
            .first(self)?;
        let expense: Option<i64> = daffodil_bills::dsl::daffodil_bills
            .select(sum(daffodil_bills::dsl::version))
            .filter(daffodil_bills::dsl::ledger_id.eq(ledger))
            .filter(daffodil_bills::dsl::currency.eq(currency))
            .filter(daffodil_bills::dsl::paid_at.ge(from))
            .filter(daffodil_bills::dsl::paid_at.lt(to))
            .filter(daffodil_bills::dsl::amount.lt(0))
            .first(self)?;
        let balance: Option<i64> = daffodil_bills::dsl::daffodil_bills
            .select(sum(daffodil_bills::dsl::version))
            .filter(daffodil_bills::dsl::ledger_id.eq(ledger))
            .filter(daffodil_bills::dsl::currency.eq(currency))
            .filter(daffodil_bills::dsl::paid_at.ge(from))
            .filter(daffodil_bills::dsl::paid_at.lt(to))
            .first(self)?;
        Ok((
            income.unwrap_or_default(),
            expense.unwrap_or_default(),
            balance.unwrap_or_default(),
        ))
    }
    fn create(
        &mut self,
        user: i32,
        ledger: i32,
        summary: &str,
        (amount, currency): (i32, &str),
        category: &str,
        (merchant, paid_at, paid_by): (&str, &NaiveDateTime, &str),
    ) -> Result<()> {
        insert_into(daffodil_bills::dsl::daffodil_bills)
            .values(&New {
                user_id: user,
                ledger_id: ledger,
                summary,
                amount,
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
        (amount, currency): (i32, &str),
        category: &str,
        (merchant, paid_at, paid_by): (&str, &NaiveDateTime, &str),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(daffodil_bills::dsl::daffodil_bills)
            .filter(daffodil_bills::dsl::id.eq(id))
            .set((
                daffodil_bills::dsl::user_id.eq(user),
                daffodil_bills::dsl::summary.eq(summary),
                daffodil_bills::dsl::amount.eq(amount),
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
    fn currencies(&mut self, ledger: i32) -> Result<Vec<String>> {
        Ok(daffodil_bills::dsl::daffodil_bills
            .select(daffodil_bills::dsl::currency)
            .filter(daffodil_bills::dsl::ledger_id.eq(ledger))
            .distinct()
            .load::<String>(self)?)
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
