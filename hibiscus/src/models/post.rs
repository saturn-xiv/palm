use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Editor, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::bbs_posts;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub forum_id: i32,
    pub topic_id: i32,
    pub parent_id: Option<i32>,
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
    fn count_by_forum(&mut self, forum: i32) -> Result<i64>;
    fn count_by_topic(&mut self, forum: i32) -> Result<i64>;
    fn count_by_author(&mut self, forum: i32) -> Result<i64>;
    fn by_forum(&mut self, forum: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_topic(&mut self, topic: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_author(&mut self, topic: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn create(
        &mut self,
        user: i32,
        forum: i32,
        topic: i32,
        parent: Option<i32>,
        body: &str,
        editor: Editor,
    ) -> Result<()>;

    fn update(&mut self, id: i32, body: &str) -> Result<()>;
    fn lock(&mut self, id: i32) -> Result<()>;
    fn unlock(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn count_by_forum(&mut self, forum: i32) -> Result<i64> {
        let it = bbs_posts::dsl::bbs_posts
            .filter(bbs_posts::dsl::forum_id.eq(forum))
            .count()
            .first(self)?;
        Ok(it)
    }
    fn count_by_topic(&mut self, topic: i32) -> Result<i64> {
        let it = bbs_posts::dsl::bbs_posts
            .filter(bbs_posts::dsl::topic_id.eq(topic))
            .count()
            .first(self)?;
        Ok(it)
    }
    fn count_by_author(&mut self, user: i32) -> Result<i64> {
        let it = bbs_posts::dsl::bbs_posts
            .filter(bbs_posts::dsl::author_id.eq(user))
            .count()
            .first(self)?;
        Ok(it)
    }
    fn by_forum(&mut self, forum: i32, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = bbs_posts::dsl::bbs_posts
            .filter(bbs_posts::dsl::forum_id.eq(forum))
            .order(bbs_posts::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_topic(&mut self, topic: i32, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = bbs_posts::dsl::bbs_posts
            .filter(bbs_posts::dsl::topic_id.eq(topic))
            .order(bbs_posts::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_author(&mut self, user: i32, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = bbs_posts::dsl::bbs_posts
            .filter(bbs_posts::dsl::author_id.eq(user))
            .order(bbs_posts::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bbs_posts::dsl::bbs_posts
            .filter(bbs_posts::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }

    fn create(
        &mut self,
        user: i32,
        forum: i32,
        topic: i32,
        parent: Option<i32>,
        body: &str,
        editor: Editor,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let editor = editor.to_string();
        insert_into(bbs_posts::dsl::bbs_posts)
            .values((
                bbs_posts::forum_id.eq(forum),
                bbs_posts::author_id.eq(user),
                bbs_posts::topic_id.eq(topic),
                bbs_posts::parent_id.eq(parent),
                bbs_posts::body.eq(body),
                bbs_posts::body_editor.eq(&editor),
                bbs_posts::status.eq(&Status::Pending.to_string()),
                bbs_posts::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(&mut self, id: i32, body: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_posts::dsl::bbs_posts.filter(bbs_posts::dsl::id.eq(id));
        update(it)
            .set((
                bbs_posts::dsl::body.eq(body),
                bbs_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn lock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_posts::dsl::bbs_posts.filter(bbs_posts::dsl::id.eq(id));
        update(it)
            .set((
                bbs_posts::dsl::locked_at.eq(&Some(now)),
                bbs_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_posts::dsl::bbs_posts.filter(bbs_posts::dsl::id.eq(id));
        update(it)
            .set((
                bbs_posts::dsl::locked_at.eq(&None::<NaiveDateTime>),
                bbs_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_posts::dsl::bbs_posts.filter(bbs_posts::dsl::id.eq(id));
        update(it)
            .set((
                bbs_posts::dsl::deleted_at.eq(&Some(now)),
                bbs_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_posts::dsl::bbs_posts.filter(bbs_posts::dsl::id.eq(id));
        update(it)
            .set((
                bbs_posts::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                bbs_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
