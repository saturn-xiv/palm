use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::{models::user::Item as User, session::current_user};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session, Editor, Error,
    HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    forum::Dao as ForumDao,
    topic::{Dao as TopicDao, Item as Topic},
};
use super::ROLE_MANAGER;

#[derive(GraphQLObject)]
#[graphql(name = "BbsTopic")]
pub struct Item {
    pub id: i32,
    pub slug: String,
    pub subject: String,
    pub body: String,
    pub body_editor: String,
    pub status: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Topic> for Item {
    fn from(it: Topic) -> Self {
        Self {
            id: it.id,
            slug: it.slug.clone(),
            subject: it.subject.clone(),
            body: it.body.clone(),
            body_editor: it.body_editor.clone(),
            status: it.status.clone(),
            locked_at: it.locked_at,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

impl Item {
    pub fn by_forum(db: &DbPool, id: i32) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        for it in TopicDao::by_forum(db, id)? {
            items.push(it.into());
        }
        Ok(items)
    }
    pub fn by_id(db: &DbPool, id: i32) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let it = TopicDao::by_id(db, id)?;
        Ok(it.into())
    }
}

#[derive(Validate, GraphQLInputObject, Clone)]
#[graphql(name = "BbsTopicCreateForm")]
pub struct Create {
    #[validate(length(min = 1, max = 127))]
    pub slug: String,
    #[validate(length(min = 1, max = 255))]
    pub subject: String,
    #[validate(length(min = 1))]
    pub body: String,
    #[validate(length(min = 1))]
    pub body_editor: String,
}

impl Create {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt, forum: i32) -> Result<()> {
        self.validate()?;
        let editor = Editor::from_str(&self.body_editor)?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        {
            let it = ForumDao::by_id(db, forum)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::GONE, None)));
            }
            if it.locked_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
            }
        }

        db.transaction::<_, Error, _>(|db| {
            TopicDao::create(
                db,
                user.id,
                forum,
                &self.slug,
                &self.subject,
                &self.body,
                editor,
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
    pub subject: String,
    #[validate(length(min = 1, max = 511))]
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

        {
            let it = TopicDao::by_id(db, id)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::GONE, None)));
            }
            if it.locked_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
            }

            it.can_edit(&user, enf).await?;
        }

        db.transaction::<_, Error, _>(|db| {
            TopicDao::update(db, id, &self.slug, &self.subject, &self.body)?;
            Ok(())
        })?;

        Ok(())
    }
}

impl Topic {
    pub async fn can_edit(&self, user: &User, enf: &Mutex<Enforcer>) -> Result<()> {
        if self.author_id == user.id {
            return Ok(());
        }
        let mut enf = enf.lock().await;
        let enf = enf.deref_mut();
        user.has(enf, ROLE_MANAGER)?;
        Ok(())
    }
}
