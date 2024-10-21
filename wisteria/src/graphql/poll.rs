use std::ops::DerefMut;

use chrono::NaiveDateTime;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session, Error, Result,
};

use super::super::models::{
    form::Dao as FormDao,
    poll::{Dao as PoolDao, Item as Pool},
};

#[derive(GraphQLObject)]
#[graphql(name = "QuestionnairePool")]
pub struct Item {
    pub id: i32,
    pub form_id: i32,
    pub batch_no: String,
    pub field_id: i32,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Pool> for Item {
    fn from(it: Pool) -> Self {
        Self {
            id: it.id,
            form_id: it.form_id,
            batch_no: it.batch_no.clone(),
            field_id: it.field_id,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

impl Item {
    pub fn by_form(ss: &Session, db: &DbPool, jwt: &Jwt, form: i32) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        let form = FormDao::by_id(db, form)?;
        form.can_edit(&user)?;

        let mut items = Vec::new();
        for it in PoolDao::by_form(db, form.id)? {
            items.push(it.into());
        }
        Ok(items)
    }
}

pub fn create(ss: &Session, db: &DbPool, jwt: &Jwt, form: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let form = FormDao::by_id(db, form)?;
    form.can_edit(&user)?;
    db.transaction::<_, Error, _>(|_db| {
        //    TODO
        Ok(())
    })?;
    Ok(())
}

pub fn update(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = PoolDao::by_id(db, id)?;
    {
        let form = FormDao::by_id(db, it.form_id)?;
        form.can_edit(&user)?;
    }
    db.transaction::<_, Error, _>(|_db| {
        // TODO
        Ok(())
    })?;
    Ok(())
}

pub fn enable(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = PoolDao::by_id(db, id)?;
    {
        let form = FormDao::by_id(db, it.form_id)?;
        form.can_edit(&user)?;
    }
    db.transaction::<_, Error, _>(|db| {
        PoolDao::enable(db, it.id)?;
        Ok(())
    })?;
    Ok(())
}
pub fn disable(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = PoolDao::by_id(db, id)?;
    {
        let form = FormDao::by_id(db, it.form_id)?;
        form.can_edit(&user)?;
    }
    db.transaction::<_, Error, _>(|db| {
        PoolDao::disable(db, it.id)?;
        Ok(())
    })?;
    Ok(())
}
