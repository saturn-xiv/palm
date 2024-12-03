use std::{ops::DerefMut, str::FromStr};

use casbin::Enforcer;
use chrono::NaiveDateTime;
use chrono_tz::Tz;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    graphql::DateTimePicker, jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool,
    session::Session, Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    ledger::Dao as LedgerDao,
    log::{Action, Dao as LogDao},
    transaction::{Dao as TransactionDao, Item as Transaction},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperTransaction")]
pub struct Item {
    pub id: i32,
    pub uid: String,
    pub ledger_id: i32,
    pub memo: String,
    pub traded_at: DateTimePicker,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn new(it: &Transaction) -> Result<Self> {
        let it = Self {
            id: it.id,
            uid: it.uid.clone(),
            ledger_id: it.ledger_id,
            memo: it.memo.clone(),
            traded_at: (it.traded_at, Tz::from_str(&it.timezone)?).try_into()?,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        };
        Ok(it)
    }
}

impl Item {
    pub async fn by_ledger(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
    ) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = LedgerDao::by_id(db, id)?;
            it.can_read(&user, enforcer).await?;
        }

        let mut items = Vec::new();

        for it in TransactionDao::by_ledger(db, id)?.iter() {
            items.push(Item::new(it)?);
        }
        Ok(items)
    }
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(max = 1023))]
    pub memo: String,
}

impl Form {
    pub async fn create(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        ledger: i32,
        (traded_at, timezone): (NaiveDateTime, Tz),
    ) -> Result<()> {
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();

        let (si, user) = current_user(ss, db, jwt)?;
        {
            let ledger = LedgerDao::by_id(db, ledger)?;
            ledger.can_append(&user, enforcer).await?;
        }

        db.transaction::<_, Error, _>(|db| {
            TransactionDao::create(db, ledger, &self.memo, traded_at, timezone)?;
            LogDao::create(
                db,
                ledger,
                (user.id, &si.to_string()),
                (
                    Action::CreateTransaction,
                    &format!("create transaction {}({})", self.memo, traded_at),
                    None,
                ),
                &ss.client_ip,
            )?;
            Ok(())
        })?;
        Ok(())
    }

    pub async fn update(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
        (traded_at, timezone): (NaiveDateTime, Tz),
    ) -> Result<()> {
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();

        let (si, user) = current_user(ss, db, jwt)?;
        let it = TransactionDao::by_id(db, id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }

        db.transaction::<_, Error, _>(|db| {
            TransactionDao::update(db, id, &self.memo, traded_at, timezone)?;
            LogDao::create(
                db,
                it.ledger_id,
                (user.id, &si.to_string()),
                (
                    Action::UpdateTransaction,
                    &format!(
                        "update transaction {}({}, {}) => {}({})",
                        it.memo, it.id, it.traded_at, self.memo, traded_at
                    ),
                    None,
                ),
                &ss.client_ip,
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

pub async fn disable(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();

    let (si, user) = current_user(ss, db, jwt)?;
    let it = TransactionDao::by_id(db, id)?;

    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        TransactionDao::disable(db, id)?;
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::CreateTransaction,
                &format!("disable transaction {}({})", it.memo, it.id),
                None,
            ),
            &ss.client_ip,
        )?;
        Ok(())
    })?;

    Ok(())
}

pub async fn enable(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();

    let (si, user) = current_user(ss, db, jwt)?;
    let it = TransactionDao::by_id(db, id)?;
    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        TransactionDao::enable(db, id)?;
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::CreateTransaction,
                &format!("enable transaction {}({})", it.memo, it.id),
                None,
            ),
            &ss.client_ip,
        )?;
        Ok(())
    })?;

    Ok(())
}
