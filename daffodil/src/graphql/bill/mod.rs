pub mod history;

use camelia::{
    models::{
        log::{Dao as LogDao, Level as LogLevel},
        user::Details as UserDetails,
    },
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};
use casbin::Enforcer;
use chrono::NaiveDateTime;
use diesel::Connection as DieselConntection;
use juniper::{GraphQLInputObject, GraphQLObject};
use log::debug;
use palm::{cache::redis::ClusterConnection as Cache, jwt::Jwt, session::Session, Error};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::{
        bill::{history::Dao as BillHistoryDao, Dao as BillDao, Item as Bill},
        ledger::{Dao as LedgerDao, Item as Ledger},
    },
    NAME,
};

#[derive(GraphQLObject)]
#[graphql(name = "DaffodilBillIndexResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub user: UserDetails,
    pub summary: String,
    pub price: i32,
    pub currency: String,
    pub merchant: String,
    pub category: String,
    pub paid_at: NaiveDateTime,
    pub paid_by: String,
    pub updated_at: NaiveDateTime,
}

impl IndexResponseItem {
    pub fn new(db: &mut Db, x: &Bill) -> Result<Self, Error> {
        let it = Self {
            id: x.id,
            user: UserDetails::new(db, x.user_id)?,
            summary: x.summary.clone(),
            price: x.price.0 as i32,
            currency: x.currency.clone(),
            merchant: x.merchant.clone(),
            category: x.category.clone(),
            paid_at: x.paid_at,
            paid_by: x.paid_by.clone(),
            updated_at: x.updated_at,
        };
        Ok(it)
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "DaffodilBillIndexResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
}

impl IndexResponse {
    pub async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        ledger: i32,
    ) -> Result<Self, Error> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        let ledger = LedgerDao::by_id(db, ledger)?;
        ledger.can_show(enf, &user).await?;

        let mut items = Vec::new();
        for it in BillDao::by_ledger(db, ledger.id)?.iter() {
            items.push(IndexResponseItem::new(db, it)?);
        }

        Ok(Self { items })
    }
}
#[derive(Validate)]
pub struct Reason<'a> {
    #[validate(length(min = 1, max = 255))]
    pub text: &'a str,
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "DaffodilBillDetailsRequest")]
pub struct Form {
    #[validate(length(min = 1, max = 511))]
    pub summary: String,
    pub price: i32,
    #[validate(length(equal = 3))]
    pub currency: String,
    #[validate(length(min = 1, max = 63))]
    pub merchant: String,
    #[validate(length(min = 1, max = 31))]
    pub category: String,
    pub paid_at: NaiveDateTime,
    #[validate(length(min = 1, max = 31))]
    pub paid_by: String,
}

impl Form {
    pub async fn create<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        ledger: i32,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        let ledger = LedgerDao::by_id(db, ledger)?;
        ledger.can_edit(enf, &user).await?;

        debug!(
            "create bill {}({}) {}({}) for {}",
            self.price, self.currency, self.paid_by, self.paid_at, ledger.name
        );
        db.transaction::<_, Error, _>(move |db| {
            BillDao::create(
                db,
                user.id,
                ledger.id,
                &self.summary,
                (self.price as i64, &self.currency),
                &self.category,
                (&self.merchant, &self.paid_at, &self.paid_by),
            )?;

            LogDao::add::<_, Ledger>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(ledger.id),
                &format!("add bill to ledger {}", ledger.name),
            )?;
            Ok(())
        })?;
        Ok(())
    }

    pub async fn update<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        (id, reason): (i32, &str),
    ) -> Result<(), Error> {
        Reason { text: reason }.validate()?;
        self.validate()?;

        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let bill = BillDao::by_id(db, id)?;

        let ledger = LedgerDao::by_id(db, bill.ledger_id)?;
        ledger.can_edit(enf, &user).await?;

        debug!(
            "update bill({}) {}({}) {}({}) for {}",
            bill.id, self.price, self.currency, self.paid_by, self.paid_at, ledger.name
        );
        db.transaction::<_, Error, _>(move |db| {
            BillDao::update(
                db,
                bill.id,
                user.id,
                &self.summary,
                (self.price as i64, &self.currency),
                &self.category,
                (&self.merchant, &self.paid_at, &self.paid_by),
            )?;
            BillHistoryDao::create(db, &bill, reason)?;
            LogDao::add::<_, Ledger>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(bill.id),
                &format!("update bill({}) {}", bill.id, bill.summary),
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

pub async fn delete<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    (id, reason): (i32, &str),
) -> Result<(), Error> {
    Reason { text: reason }.validate()?;

    let (user, _, _) = ss.current_user(db, ch, jwt)?;

    let bill = BillDao::by_id(db, id)?;

    let ledger = LedgerDao::by_id(db, bill.ledger_id)?;
    ledger.can_edit(enf, &user).await?;

    debug!("delete bill({}) for {}", bill.id, ledger.name);
    db.transaction::<_, Error, _>(move |db| {
        BillDao::delete(db, bill.id)?;
        BillHistoryDao::create(db, &bill, reason)?;
        LogDao::add::<_, Ledger>(
            db,
            user.id,
            NAME,
            LogLevel::Info,
            &ss.client_ip,
            Some(bill.id),
            &format!("delete bill({}) {}", bill.id, bill.summary),
        )?;
        Ok(())
    })?;
    Ok(())
}

pub fn merchants<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    jwt: &J,
) -> Result<Vec<String>, Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    let items = BillDao::merchants(db, user.id)?;
    Ok(items)
}

pub fn categories<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    jwt: &J,
) -> Result<Vec<String>, Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    let items = BillDao::categories(db, user.id)?;
    Ok(items)
}

pub fn payment_methods<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    jwt: &J,
) -> Result<Vec<String>, Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    let items = BillDao::payment_methods(db, user.id)?;
    Ok(items)
}
