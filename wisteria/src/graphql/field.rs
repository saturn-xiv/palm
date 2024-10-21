use std::ops::DerefMut;

use chrono::NaiveDateTime;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session, Error, Result,
};
use validator::Validate;

use super::super::models::{
    field::{Dao as FieldDao, Item as Field},
    form::Dao as FormDao,
};

#[derive(GraphQLObject)]
#[graphql(name = "QuestionnaireField")]
pub struct Item {
    pub id: i32,
    pub form_id: i32,
    pub uid: String,
    pub label: String,
    pub summary: String,
    pub sort_order: i32,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Field> for Item {
    fn from(it: Field) -> Self {
        Self {
            id: it.id,
            form_id: it.form_id,
            uid: it.uid.clone(),
            label: it.label.clone(),
            summary: it.summary.clone(),
            sort_order: it.sort_order,
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
        for it in FieldDao::by_form(db, form.id)? {
            items.push(it.into());
        }
        Ok(items)
    }
}

#[derive(Validate)]
pub struct Create {
    #[validate(length(min = 1, max = 255))]
    pub label: String,
    #[validate(length(min = 1, max = 1023))]
    pub summary: String,
    #[validate(range(min = -99, max = 99))]
    pub sort_order: i32,
}

impl Create {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt, form: i32) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        let form = FormDao::by_id(db, form)?;
        form.can_edit(&user)?;
        db.transaction::<_, Error, _>(|db| {
            FieldDao::create(
                db,
                user.id,
                form.id,
                &self.label,
                &self.summary,
                self.sort_order,
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Validate)]
pub struct Update {
    #[validate(length(min = 1, max = 255))]
    pub label: String,
    #[validate(length(min = 1, max = 1023))]
    pub summary: String,
    #[validate(range(min = -99, max = 99))]
    pub sort_order: i32,
}

impl Update {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        let it = FieldDao::by_id(db, id)?;
        {
            let form = FormDao::by_id(db, it.form_id)?;
            form.can_edit(&user)?;
        }
        db.transaction::<_, Error, _>(|db| {
            FieldDao::update(db, it.id, &self.label, &self.summary, self.sort_order)?;
            Ok(())
        })?;
        Ok(())
    }
}
pub fn enable(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = FieldDao::by_id(db, id)?;
    {
        let form = FormDao::by_id(db, it.form_id)?;
        form.can_edit(&user)?;
    }
    db.transaction::<_, Error, _>(|db| {
        FieldDao::enable(db, it.id)?;
        Ok(())
    })?;
    Ok(())
}
pub fn disable(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = FieldDao::by_id(db, id)?;
    {
        let form = FormDao::by_id(db, it.form_id)?;
        form.can_edit(&user)?;
    }
    db.transaction::<_, Error, _>(|db| {
        FieldDao::disable(db, it.id)?;
        Ok(())
    })?;
    Ok(())
}
