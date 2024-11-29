use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::{models::user::Item as User, session::current_user};
use diesel::Connection as DieselConnection;
use juniper::{GraphQLInputObject, GraphQLObject};
use language_tags::LanguageTag;
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Editor, Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::page::{Dao as PageDao, Item as Page, Template};
use super::ROLE_MANAGER;

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
    #[validate(length(min = 1, max = 127))]
    pub slug: String,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub body: String,
    pub body_editor: Editor,
    pub template: Template,
}

impl Create {
    pub async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enf: &Mutex<Enforcer>,
        lang: &str,
    ) -> Result<()> {
        let lang = {
            let it = LanguageTag::from_str(lang)?;
            it.to_string()
        };
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        {
            let mut enf = enf.lock().await;
            let enf = enf.deref_mut();
            user.has(enf, ROLE_MANAGER)?;
        }

        db.transaction::<_, Error, _>(|db| {
            PageDao::create(
                db,
                user.id,
                &lang,
                &self.slug,
                &self.title,
                (&self.body, self.body_editor, self.template),
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
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub body: String,
}

impl Update {
    pub async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enf: &Mutex<Enforcer>,
        id: i32,
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        let it = PageDao::by_id(db, id)?;
        it.can_edit(&user, enf).await?;

        db.transaction::<_, Error, _>(|db| {
            PageDao::update(db, it.id, &self.slug, &self.title, &self.body)?;
            Ok(())
        })?;

        Ok(())
    }
}

pub async fn set_template(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enf: &Mutex<Enforcer>,
    id: i32,
    template: Template,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = PageDao::by_id(db, id)?;
    it.can_edit(&user, enf).await?;

    db.transaction::<_, Error, _>(|db| {
        PageDao::set_template(db, it.id, template)?;
        Ok(())
    })?;

    Ok(())
}

impl Page {
    pub async fn can_edit(&self, user: &User, enf: &Mutex<Enforcer>) -> Result<()> {
        if self.user_id == user.id {
            return Ok(());
        }
        let mut enf = enf.lock().await;
        let enf = enf.deref_mut();
        user.has(enf, ROLE_MANAGER)?;
        Ok(())
    }
}
