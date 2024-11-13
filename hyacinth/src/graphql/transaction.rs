use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session, Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    ledger::Dao as LedgerDao,
    transaction::{Dao as TransactionDao, Item as Transaction},
};
use super::entry::New as NewEntryForm;

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperTransaction")]
pub struct Item {
    pub id: i32,
    pub uid: String,
    pub ledger_id: i32,
    pub memo: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl From<Transaction> for Item {
    fn from(it: Transaction) -> Self {
        Self {
            id: it.id,
            uid: it.uid.clone(),
            ledger_id: it.ledger_id,
            memo: it.memo.clone(),
            deleted_at: it.deleted_at,
            created_at: it.created_at,
        }
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

        for it in TransactionDao::by_ledger(db, id)? {
            items.push(it.into());
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
        entries: &[NewEntryForm],
    ) -> Result<()> {
        self.validate()?;
        for it in entries.iter() {
            it.validate()?;
        }
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let ledger = LedgerDao::by_id(db, ledger)?;
            ledger.can_append(&user, enforcer).await?;
        }

        db.transaction::<_, Error, _>(|db| {
            let uid = TransactionDao::create(db, ledger, &self.memo)?;
            let it = TransactionDao::by_uid(db, &uid)?;
            for ie in entries.iter() {
                ie.save(db, &it)?;
            }
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

    {
        let (_, user) = current_user(ss, db, jwt)?;
        let it = TransactionDao::by_id(db, id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
    }

    db.transaction::<_, Error, _>(|db| {
        TransactionDao::disable(db, id)?;
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
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let it = TransactionDao::by_id(db, id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
    }

    db.transaction::<_, Error, _>(|db| {
        TransactionDao::enable(db, id)?;
        Ok(())
    })?;

    Ok(())
}
