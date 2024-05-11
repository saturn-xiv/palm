use std::any::type_name;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use palm::{azalea::v1::TextEditor, Result};
use serde::Serialize;

use super::super::super::{orm::postgresql::Connection, schema::vote_logs};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub ip: String,
    pub star: i32,
    pub comment: String,
    pub comment_editor: String,
    pub resource_type: String,
    pub resource_id: i64,
    pub status: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
pub trait Dao {
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_resource(&mut self, type_: &str, id: i64) -> Result<Vec<Item>>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count(&mut self) -> Result<i64>;
    fn by_user(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count_by_user(&mut self, user: i64) -> Result<i64>;
    fn create(
        &mut self,
        user: i64,
        ip: &str,
        star: i32,
        comment: (&str, TextEditor),
        resource: (&str, i64),
    ) -> Result<()>;
    fn create_<T>(
        &mut self,
        user: i64,
        ip: &str,
        star: i32,
        comment: (&str, TextEditor),
        resource_id: i64,
    ) -> Result<()>;
    fn update(&mut self, id: i64, star: i32, comment: &str) -> Result<()>;
    fn destroy(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = vote_logs::dsl::vote_logs
            .filter(vote_logs::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_resource(&mut self, type_: &str, id: i64) -> Result<Vec<Item>> {
        let items = vote_logs::dsl::vote_logs
            .filter(vote_logs::dsl::resource_type.eq(type_))
            .filter(vote_logs::dsl::resource_id.eq(id))
            .order(vote_logs::dsl::updated_at.desc())
            .load(self)?;
        Ok(items)
    }
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = vote_logs::dsl::vote_logs
            .order(vote_logs::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load(self)?;
        Ok(items)
    }
    fn count(&mut self) -> Result<i64> {
        let it = vote_logs::dsl::vote_logs.count().first(self)?;
        Ok(it)
    }
    fn by_user(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = vote_logs::dsl::vote_logs
            .filter(vote_logs::dsl::user_id.eq(user))
            .order(vote_logs::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load(self)?;
        Ok(items)
    }
    fn count_by_user(&mut self, user: i64) -> Result<i64> {
        let it = vote_logs::dsl::vote_logs
            .filter(vote_logs::dsl::user_id.eq(user))
            .count()
            .first(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        ip: &str,
        star: i32,
        (comment, editor): (&str, TextEditor),
        (resource_type, resource_id): (&str, i64),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();

        insert_into(vote_logs::dsl::vote_logs)
            .values((
                vote_logs::dsl::user_id.eq(user),
                vote_logs::dsl::ip.eq(ip),
                vote_logs::dsl::star_.eq(star),
                vote_logs::dsl::comment.eq(comment),
                vote_logs::dsl::comment_editor.eq(editor.as_str_name()),
                vote_logs::dsl::resource_id.eq(resource_id),
                vote_logs::dsl::resource_type.eq(resource_type),
                vote_logs::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn create_<T>(
        &mut self,
        user: i64,
        ip: &str,
        star: i32,
        comment: (&str, TextEditor),
        resource_id: i64,
    ) -> Result<()> {
        Self::create(
            self,
            user,
            ip,
            star,
            comment,
            (type_name::<T>(), resource_id),
        )
    }
    fn update(&mut self, id: i64, star: i32, comment: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(vote_logs::dsl::vote_logs.filter(vote_logs::dsl::id.eq(id)))
            .set((
                vote_logs::dsl::star_.eq(star),
                vote_logs::dsl::comment.eq(comment),
                vote_logs::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn destroy(&mut self, id: i64) -> Result<()> {
        delete(vote_logs::dsl::vote_logs.filter(vote_logs::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
