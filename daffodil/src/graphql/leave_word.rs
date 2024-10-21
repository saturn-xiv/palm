use std::{ops::DerefMut, str::FromStr};

use casbin::Enforcer;
use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Editor, Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::leave_word::{Dao as LeaveWordDao, Item as LeaveWord, Status as LeaveWordStatus},
    session::current_user,
};

#[derive(GraphQLObject)]
#[graphql(name = "LeaveWord")]
pub struct Item {
    pub id: i32,
    pub lang: String,
    pub ip: String,
    pub body: String,
    pub body_editor: String,
    pub status: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<LeaveWord> for Item {
    fn from(it: LeaveWord) -> Self {
        Self {
            id: it.id,
            lang: it.lang.clone(),
            ip: it.ip.clone(),
            body: it.body.clone(),
            body_editor: it.body_editor.clone(),
            status: it.status.clone(),
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "LeaveWordList")]
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
        let total = LeaveWordDao::count(db)?;
        let pagination = Pagination::new(pager, total);
        for it in LeaveWordDao::index(db, pager.offset(total), pager.size())? {
            items.push(it.into());
        }
        Ok(Self { items, pagination })
    }
}

pub async fn destroy(
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
        LeaveWordDao::disable(db, id)?;
        Ok(())
    })?;

    Ok(())
}

pub async fn close(
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
        LeaveWordDao::set_status(db, id, LeaveWordStatus::Closed)?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Validate)]
pub struct Create {
    #[validate(length(min = 31))]
    pub body: String,
    #[validate(length(min = 1, max = 15))]
    pub editor: String,
}

impl Create {
    pub fn execute(&self, ss: &Session, db: &DbPool) -> Result<()> {
        self.validate()?;
        let editor = Editor::from_str(&self.editor)?;
        let mut db = db.get()?;
        let db = db.deref_mut();

        db.transaction::<_, Error, _>(|db| {
            LeaveWordDao::create(db, &ss.lang, &ss.client_ip, &self.body, editor)?;
            Ok(())
        })?;
        Ok(())
    }
}
