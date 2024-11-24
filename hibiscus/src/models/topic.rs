use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Editor, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::bbs_topics;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub forum_id: i32,
    pub slug: String,
    pub subject: String,
    pub body: String,
    pub body_editor: String,
    pub author_id: i32,
    pub status: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    #[default]
    Pending,
    Opening,
    Closed,
}

pub trait Dao {
    fn by_forum(&mut self, forum: i32) -> Result<Vec<Item>>;
    fn by_author(&mut self, user: i32) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_slug(&mut self, slug: &str) -> Result<Item>;
    fn create(
        &mut self,
        user: i32,
        forum: i32,
        slug: &str,
        subject: &str,
        body: &str,
        editor: Editor,
    ) -> Result<()>;
    fn update(&mut self, id: i32, slug: &str, subject: &str, body: &str) -> Result<()>;
    fn lock(&mut self, id: i32) -> Result<()>;
    fn unlock(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_forum(&mut self, forum: i32) -> Result<Vec<Item>> {
        let items = bbs_topics::dsl::bbs_topics
            .filter(bbs_topics::dsl::forum_id.eq(forum))
            .order(bbs_topics::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_author(&mut self, user: i32) -> Result<Vec<Item>> {
        let items = bbs_topics::dsl::bbs_topics
            .filter(bbs_topics::dsl::author_id.eq(user))
            .order(bbs_topics::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bbs_topics::dsl::bbs_topics
            .filter(bbs_topics::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_slug(&mut self, slug: &str) -> Result<Item> {
        let it = bbs_topics::dsl::bbs_topics
            .filter(bbs_topics::dsl::slug.eq(slug))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i32,
        forum: i32,
        slug: &str,
        subject: &str,
        body: &str,
        editor: Editor,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let editor = editor.to_string();
        insert_into(bbs_topics::dsl::bbs_topics)
            .values((
                bbs_topics::author_id.eq(user),
                bbs_topics::forum_id.eq(forum),
                bbs_topics::slug.eq(slug),
                bbs_topics::subject.eq(subject),
                bbs_topics::body.eq(body),
                bbs_topics::body_editor.eq(&editor),
                bbs_topics::status.eq(&Status::Pending.to_string()),
                bbs_topics::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, slug: &str, subject: &str, body: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_topics::dsl::bbs_topics.filter(bbs_topics::dsl::id.eq(id));
        update(it)
            .set((
                bbs_topics::dsl::slug.eq(slug),
                bbs_topics::dsl::subject.eq(subject),
                bbs_topics::dsl::body.eq(body),
                bbs_topics::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn lock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_topics::dsl::bbs_topics.filter(bbs_topics::dsl::id.eq(id));
        update(it)
            .set((
                bbs_topics::dsl::locked_at.eq(&Some(now)),
                bbs_topics::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_topics::dsl::bbs_topics.filter(bbs_topics::dsl::id.eq(id));
        update(it)
            .set((
                bbs_topics::dsl::locked_at.eq(&None::<NaiveDateTime>),
                bbs_topics::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_topics::dsl::bbs_topics.filter(bbs_topics::dsl::id.eq(id));
        update(it)
            .set((
                bbs_topics::dsl::deleted_at.eq(&Some(now)),
                bbs_topics::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_topics::dsl::bbs_topics.filter(bbs_topics::dsl::id.eq(id));
        update(it)
            .set((
                bbs_topics::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                bbs_topics::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
