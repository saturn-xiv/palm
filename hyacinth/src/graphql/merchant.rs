use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::{
    models::postal::{
        address::{Dao as AddressDao, Item as Address},
        recipient::{Dao as RecipientDao, Item as Recipient},
    },
    session::current_user,
};
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    session::Session,
    Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    ledger::Dao as LedgerDao,
    merchant::{Dao as MerchantDao, Item as Merchant},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperMerchant")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub label: String,
    pub memo: String,
    pub address: Option<Address>,
    pub contact: Option<Recipient>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn new(db: &mut Db, it: &Merchant) -> Result<Self> {
        let it = Self {
            id: it.id,
            ledger_id: it.ledger_id,
            label: it.label.clone(),
            memo: it.memo.clone(),
            contact: match it.contact {
                Some(id) => Some(RecipientDao::by_id(db, id)?),
                None => None,
            },
            address: match it.address {
                Some(id) => Some(AddressDao::by_id(db, id)?),
                None => None,
            },
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        };
        Ok(it)
    }
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

        for it in MerchantDao::by_ledger(db, id)?.iter() {
            items.push(Self::new(db, it)?);
        }
        Ok(items)
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
        let it = MerchantDao::by_id(db, id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
    }

    db.transaction::<_, Error, _>(|db| {
        MerchantDao::disable(db, id)?;
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
        let it = MerchantDao::by_id(db, id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
    }

    db.transaction::<_, Error, _>(|db| {
        MerchantDao::enable(db, id)?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 63))]
    pub label: String,
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
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let ledger = LedgerDao::by_id(db, ledger)?;
            ledger.can_append(&user, enforcer).await?;
        }

        db.transaction::<_, Error, _>(|db| {
            MerchantDao::create(db, ledger, &self.label, &self.memo)?;
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
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = MerchantDao::by_id(db, id)?;
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
        db.transaction::<_, Error, _>(|db| {
            MerchantDao::set_details(db, id, &self.label, &self.memo)?;
            Ok(())
        })?;
        Ok(())
    }
}
