use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::{models::user::Item as User, session::current_user};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Editor, Error, HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    forum::Dao as ForumDao,
    post::{Dao as PostDao, Item as Post},
    topic::Dao as TopicDao,
};
use super::ROLE_MANAGER;

#[derive(GraphQLObject)]
#[graphql(name = "BbsPost")]
pub struct Item {
    pub id: i32,
    pub body: String,
    pub body_editor: String,
    pub status: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Post> for Item {
    fn from(it: Post) -> Self {
        Self {
            id: it.id,
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
    pub fn by_id(db: &DbPool, id: i32) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let it = PostDao::by_id(db, id)?;
        Ok(it.into())
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "BbsPostList")]
pub struct List {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

impl List {
    pub fn by_forum(db: &DbPool, id: i32, pager: Pager) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        let total = PostDao::count_by_forum(db, id)?;
        let pagination = Pagination::new(&pager, total);
        for it in PostDao::by_forum(db, id, pager.offset(total), pager.size())? {
            items.push(it.into());
        }
        Ok(Self { items, pagination })
    }
    pub fn by_topic(db: &DbPool, id: i32, pager: Pager) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        let total = PostDao::count_by_topic(db, id)?;
        let pagination = Pagination::new(&pager, total);
        for it in PostDao::by_topic(db, id, pager.offset(total), pager.size())? {
            items.push(it.into());
        }
        Ok(Self { items, pagination })
    }
}

#[derive(Validate, GraphQLInputObject, Clone)]
#[graphql(name = "BbsPostCreateForm")]
pub struct Create {
    #[validate(length(min = 1))]
    pub body: String,
    #[validate(length(min = 1))]
    pub body_editor: String,
}

impl Create {
    pub fn reply(&self, ss: &Session, db: &DbPool, jwt: &Jwt, parent: i32) -> Result<()> {
        self.validate()?;
        let editor = Editor::from_str(&self.body_editor)?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        let parent = PostDao::by_id(db, parent)?;
        if parent.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::GONE, None)));
        }
        if parent.locked_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
        }

        {
            let topic = TopicDao::by_id(db, parent.topic_id)?;
            if topic.deleted_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::GONE, None)));
            }
            if topic.locked_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
            }

            let forum = ForumDao::by_id(db, topic.forum_id)?;
            if forum.deleted_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::GONE, None)));
            }
            if forum.locked_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
            }
        };

        db.transaction::<_, Error, _>(|db| {
            PostDao::create(
                db,
                user.id,
                parent.forum_id,
                parent.topic_id,
                Some(parent.id),
                &self.body,
                editor,
            )?;
            Ok(())
        })?;

        Ok(())
    }
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt, topic: i32) -> Result<()> {
        self.validate()?;
        let editor = Editor::from_str(&self.body_editor)?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        let forum = {
            let topic = TopicDao::by_id(db, topic)?;
            if topic.deleted_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::GONE, None)));
            }
            if topic.locked_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
            }

            let forum = ForumDao::by_id(db, topic.forum_id)?;
            if forum.deleted_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::GONE, None)));
            }
            if forum.locked_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
            }
            forum.id
        };

        db.transaction::<_, Error, _>(|db| {
            PostDao::create(db, user.id, forum, topic, None, &self.body, editor)?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate)]
pub struct Update {
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
            let it = PostDao::by_id(db, id)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::GONE, None)));
            }
            if it.locked_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
            }

            it.can_edit(&user, enf).await?;
        }

        db.transaction::<_, Error, _>(|db| {
            PostDao::update(db, id, &self.body)?;
            Ok(())
        })?;

        Ok(())
    }
}

impl Post {
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
