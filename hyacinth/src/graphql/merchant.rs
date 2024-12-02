use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::{
    models::postal::{
        address::{Dao as AddressDao, Form as AddressForm, Item as Address},
        recipient::{Dao as RecipientDao, Form as RecipientForm, Item as Recipient},
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
    log::{Action, Dao as LogDao},
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
    let (si, user) = current_user(ss, db, jwt)?;
    let it = MerchantDao::by_id(db, id)?;

    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        MerchantDao::disable(db, id)?;
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::DisableMerchant,
                &format!("disable merchant {}({})", it.label, it.id),
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
    let it = MerchantDao::by_id(db, id)?;
    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        MerchantDao::enable(db, id)?;
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::EnableMerchant,
                &format!("disable merchant {}({})", it.label, it.id),
                None,
            ),
            &ss.client_ip,
        )?;
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

        let (si, user) = current_user(ss, db, jwt)?;
        {
            let ledger = LedgerDao::by_id(db, ledger)?;
            ledger.can_append(&user, enforcer).await?;
        }

        db.transaction::<_, Error, _>(|db| {
            let id = MerchantDao::create(db, ledger, &self.label, &self.memo)?;
            LogDao::create(
                db,
                ledger,
                (user.id, &si.to_string()),
                (
                    Action::DisableMerchant,
                    &format!("create merchant {} {}({})", self.label, self.memo, id),
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
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();

        let (si, user) = current_user(ss, db, jwt)?;
        let it = MerchantDao::by_id(db, id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
        db.transaction::<_, Error, _>(|db| {
            MerchantDao::set_details(db, id, &self.label, &self.memo)?;
            LogDao::create(
                db,
                it.ledger_id,
                (user.id, &si.to_string()),
                (
                    Action::DisableMerchant,
                    &format!("update merchant({}) =>{} {}", id, self.label, self.memo),
                    None,
                ),
                &ss.client_ip,
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

pub async fn set_contact(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    form: &RecipientForm,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (si, user) = current_user(ss, db, jwt)?;
    let it = MerchantDao::by_id(db, id)?;

    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        MerchantDao::disable(db, id)?;
        match it.contact {
            Some(contact) => {
                RecipientDao::update(db, contact, form)?;
            }
            None => {
                let rid = RecipientDao::create(db, form)?;
                MerchantDao::set_contact(db, id, rid)?;
            }
        }
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::UpdateMerchant,
                &format!(
                    "update merchant {}({})'s contact {:?}",
                    it.label, it.id, form
                ),
                None,
            ),
            &ss.client_ip,
        )?;
        Ok(())
    })?;

    Ok(())
}

pub async fn set_address(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    form: &AddressForm,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (si, user) = current_user(ss, db, jwt)?;
    let it = MerchantDao::by_id(db, id)?;

    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        MerchantDao::disable(db, id)?;
        match it.address {
            Some(address) => {
                AddressDao::update(db, address, form)?;
            }
            None => {
                let aid = AddressDao::create(db, form)?;
                MerchantDao::set_address(db, id, aid)?;
            }
        }
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::UpdateMerchant,
                &format!(
                    "update merchant {}({})'s address {:?}",
                    it.label, it.id, form
                ),
                None,
            ),
            &ss.client_ip,
        )?;
        Ok(())
    })?;

    Ok(())
}
