use std::{ops::DerefMut, str::FromStr};

use chrono::NaiveDateTime;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use language_tags::LanguageTag;
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Editor, Error, HttpError, Result,
};
use validator::Validate;

use super::super::models::page::{Dao as PageDao, Item as Page};

#[derive(GraphQLObject)]
#[graphql(name = "CmsPage")]
pub struct Item {
    pub id: i32,
    pub lang: String,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub body_editor: String,
    pub template: String,
    pub status: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Page> for Item {
    fn from(it: Page) -> Self {
        Self {
            id: it.id,
            lang: it.lang.clone(),
            slug: it.slug.clone(),
            title: it.title.clone(),
            body: it.body.clone(),
            body_editor: it.body_editor.clone(),
            template: it.template.clone(),
            status: it.status.clone(),
            locked_at: it.locked_at,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "CmsPageList")]
pub struct List {
    pub pagination: Pagination,
    pub items: Vec<Item>,
}

impl List {
    pub fn new(ss: &Session, db: &DbPool, jwt: &Jwt, pager: &Pager) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        let mut items = Vec::new();
        let total = PageDao::count_by_user(db, user.id)?;
        let pagination = Pagination::new(pager, total);
        for it in PageDao::index_by_user(db, user.id, pager.offset(total), pager.size())? {
            items.push(it.into());
        }
        Ok(Self { items, pagination })
    }
}

#[derive(Validate, GraphQLInputObject, Clone)]
#[graphql(name = "CmsPageCreateForm")]
pub struct Create {
    #[validate(length(min = 1, max = 15))]
    pub lang: String,
    #[validate(length(min = 1, max = 127))]
    pub slug: String,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub body: String,
    #[validate(length(min = 1, max = 15))]
    pub body_editor: String,
    #[validate(length(min = 1, max = 31))]
    pub template: String,
}

impl Create {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt) -> Result<()> {
        let lang = {
            let it = LanguageTag::from_str(&self.lang)?;
            it.to_string()
        };
        let editor = Editor::from_str(&self.body_editor)?;
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        db.transaction::<_, Error, _>(|db| {
            PageDao::create(
                db,
                user.id,
                &lang,
                &self.slug,
                &self.body,
                editor,
                &self.template,
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate)]
pub struct Update {
    #[validate(length(min = 1, max = 127))]
    pub slug: String,
    #[validate(length(min = 1))]
    pub body: String,
}

impl Update {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        let it = PageDao::by_id(db, id)?;
        if it.user_id != user.id {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }

        db.transaction::<_, Error, _>(|db| {
            PageDao::update(db, it.id, &self.slug, &self.body)?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate)]
pub struct SetTemplate {
    #[validate(length(min = 1, max = 31))]
    pub template: String,
}

impl SetTemplate {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        let it = PageDao::by_id(db, id)?;
        if it.user_id != user.id {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }

        db.transaction::<_, Error, _>(|db| {
            PageDao::set_template(db, it.id, &self.template)?;
            Ok(())
        })?;

        Ok(())
    }
}
