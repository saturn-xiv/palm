use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::{models::user::Item as User, session::current_user};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    rbac::v1::policy_permissions_response::item::{Operation, Resource},
    session::Session,
    Error, HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::{
    super::models::{
        ledger::{Dao as LedgerDao, Item as Ledger},
        log::{Action, Dao as LogDao},
    },
    ROLE_MEMBER,
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperLedger")]
pub struct Item {
    pub id: i32,
    pub uid: String,
    pub label: String,
    pub memo: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Ledger> for Item {
    fn from(it: Ledger) -> Self {
        Self {
            id: it.id,
            uid: it.uid.clone(),
            label: it.label.clone(),
            memo: it.memo.clone(),
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

impl Item {
    pub async fn by_id(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        let it = LedgerDao::by_id(db, id)?;
        it.can_read(&user, enforcer).await?;

        Ok(it.into())
    }
    pub async fn all(ss: &Session, db: &DbPool, jwt: &Jwt) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        let mut items = Vec::new();

        for it in LedgerDao::by_user(db, user.id)? {
            items.push(it.into());
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

    let it = LedgerDao::by_id(db, id)?;
    it.can_write(&user, enforcer).await?;

    db.transaction::<_, Error, _>(|db| {
        LedgerDao::disable(db, id)?;
        LogDao::create(
            db,
            it.id,
            (user.id, &si.to_string()),
            (
                Action::DisableLedge,
                &format!("disable {}({})", it.label, id),
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

    let it = LedgerDao::by_id(db, id)?;
    it.can_write(&user, enforcer).await?;

    db.transaction::<_, Error, _>(|db| {
        LedgerDao::enable(db, id)?;
        LogDao::create(
            db,
            id,
            (user.id, &si.to_string()),
            (
                Action::EnableLedge,
                &format!("enable {}({})", it.label, id),
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
    pub fn create(&self, ss: &Session, db: &DbPool, jwt: &Jwt) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (si, user) = current_user(ss, db, jwt)?;

        db.transaction::<_, Error, _>(|db| {
            let uid = LedgerDao::create(db, user.id, &self.label, &self.memo)?;
            let it = LedgerDao::by_uid(db, &uid)?;
            LogDao::create(
                db,
                it.id,
                (user.id, &si.to_string()),
                (
                    Action::CreateLedge,
                    &format!("{} {}", self.label, self.memo),
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

        let it = LedgerDao::by_id(db, id)?;
        it.can_write(&user, enforcer).await?;

        db.transaction::<_, Error, _>(|db| {
            LedgerDao::set_details(db, id, &self.label, &self.memo)?;
            LogDao::create(
                db,
                id,
                (user.id, &si.to_string()),
                (
                    Action::UpdateLedge,
                    &format!(
                        "{} => {}, {} => {}",
                        it.label, self.label, it.memo, self.memo
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

impl Ledger {
    pub async fn can_write(&self, user: &User, enforcer: &Mutex<Enforcer>) -> Result<()> {
        if user.id == self.user_id {
            return Ok(());
        }
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        {
            user.has(enf, ROLE_MEMBER)?;
            user.can(enf, &Operation::write(), &Resource::by_id::<Self>(self.id))?;
        }
        Ok(())
    }
    pub async fn can_read(&self, user: &User, enforcer: &Mutex<Enforcer>) -> Result<()> {
        if user.id == self.user_id {
            return Ok(());
        }
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        {
            user.has(enf, ROLE_MEMBER)?;
            user.can(enf, &Operation::read(), &Resource::by_id::<Self>(self.id))?;
        }
        Ok(())
    }
    pub async fn can_append(&self, user: &User, enforcer: &Mutex<Enforcer>) -> Result<()> {
        if user.id == self.user_id {
            return Ok(());
        }
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::GONE, None)));
        }
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        {
            user.has(enf, ROLE_MEMBER)?;
            user.can(enf, &Operation::append(), &Resource::by_id::<Self>(self.id))?;
        }
        Ok(())
    }
}
