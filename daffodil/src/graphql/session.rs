use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Error, Result,
};
use tokio::sync::Mutex;

use super::super::{
    models::{session::Dao as SessionDao, user::Dao as UserDao},
    session::current_user,
};
use super::user::Item as UserDetail;

#[derive(GraphQLObject)]
#[graphql(name = "Session")]
pub struct Item {
    pub id: i32,
    pub user: UserDetail,
    pub uid: String,
    pub provider_type: String,
    pub provider_id: i32,
    pub ip: String,
    pub expires_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(GraphQLObject)]
#[graphql(name = "SessionList")]
pub struct List {
    pub pagination: Pagination,
    pub items: Vec<Item>,
}

impl List {
    pub async fn new(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        pager: &Pager,
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            user.is_administrator(enf)?;
        }
        let mut items = Vec::new();
        let total = SessionDao::total(db)?;
        let pagination = Pagination::new(pager, total);
        for it in SessionDao::index(db, pager.offset(total), pager.size())?.iter() {
            let user = UserDao::by_id(db, it.user_id)?;
            items.push(Item {
                id: it.id,
                user: user.into(),
                uid: it.uid.clone(),
                provider_type: it.provider_type.clone(),
                provider_id: it.provider_id,
                ip: it.ip.clone(),
                expires_at: it.expires_at,
                deleted_at: it.deleted_at,
                created_at: it.created_at,
            });
        }
        Ok(Self { items, pagination })
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
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }
    db.transaction::<_, Error, _>(|db| {
        SessionDao::disable(db, id)?;
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
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }
    db.transaction::<_, Error, _>(|db| {
        SessionDao::enable(db, id)?;
        Ok(())
    })?;
    Ok(())
}
