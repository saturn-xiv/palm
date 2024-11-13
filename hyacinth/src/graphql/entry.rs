use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    session::Session,
    Error, HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    account::Dao as AccountDao,
    category::Dao as CategoryDao,
    entry::{Dao as EntryDao, Item as Entry},
    ledger::Dao as LedgerDao,
    merchant::Dao as MerchantDao,
    transaction::{Dao as TransactionDao, Item as Transaction},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperEntry")]
pub struct Item {
    pub id: i32,
    pub transaction: i32,
    pub from_account: i32,
    pub to_account: i32,
    pub category: i32,
    pub merchant: i32,
    pub amount: i32,
    pub memo: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl From<Entry> for Item {
    fn from(it: Entry) -> Self {
        Self {
            id: it.id,
            transaction: it.transaction_id,
            from_account: it.from_account_id,
            to_account: it.to_account_id,
            category: it.category_id,
            merchant: it.merchant_id,
            memo: it.memo.clone(),
            amount: it.amount,
            deleted_at: it.deleted_at,
            created_at: it.created_at,
        }
    }
}

impl Item {
    pub async fn by_transaction(
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

        for it in EntryDao::by_transaction(db, id)? {
            items.push(it.into());
        }
        Ok(items)
    }
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "NewBookkeeperEntryForm")]
pub struct New {
    pub from_account: i32,
    pub to_account: i32,
    pub category: i32,
    pub merchant: i32,
    pub amount: i32,
    #[validate(length(max = 1023))]
    pub memo: String,
}

impl New {
    pub async fn create(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        transaction: i32,
    ) -> Result<()> {
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let it = {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = TransactionDao::by_id(db, transaction)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("transaction is disabled".to_string()),
                )));
            }
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
            it
        };

        db.transaction::<_, Error, _>(|db| {
            self.save(db, &it)?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn save(&self, db: &mut Db, transaction: &Transaction) -> Result<()> {
        {
            let it = AccountDao::by_id(db, self.from_account)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("from account is disabled".to_string()),
                )));
            }
            if it.ledger_id != transaction.ledger_id {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("from account's ledger not match".to_string()),
                )));
            }
        }
        {
            let it = AccountDao::by_id(db, self.to_account)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("to account is disabled".to_string()),
                )));
            }
            if it.ledger_id != transaction.ledger_id {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("to account's ledger not match".to_string()),
                )));
            }
        }
        {
            let it = CategoryDao::by_id(db, self.category)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("category is disabled".to_string()),
                )));
            }
            if it.ledger_id != transaction.ledger_id {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("category's ledger not match".to_string()),
                )));
            }
        }
        {
            let it = MerchantDao::by_id(db, self.merchant)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("merchant is disabled".to_string()),
                )));
            }
            if it.ledger_id != transaction.ledger_id {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("merchant's ledger not match".to_string()),
                )));
            }
        }

        EntryDao::create(
            db,
            transaction.id,
            self.category,
            (self.from_account, self.to_account),
            (self.merchant, self.amount, &self.memo),
        )?;
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
        let ie = EntryDao::by_id(db, id)?;
        let it = TransactionDao::by_id(db, ie.transaction_id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
    }

    db.transaction::<_, Error, _>(|db| {
        EntryDao::disable(db, id)?;
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
        let ie = EntryDao::by_id(db, id)?;
        let it = TransactionDao::by_id(db, ie.transaction_id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
    }

    db.transaction::<_, Error, _>(|db| {
        EntryDao::enable(db, id)?;
        Ok(())
    })?;

    Ok(())
}
