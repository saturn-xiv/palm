use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use serde::Serialize;

use super::super::super::{orm::postgresql::Connection, Result};
use super::{
    schema::{groups, groups_users, roles_groups},
    user::{Dao as UserDao, Item as User},
};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub parent_id: Option<i64>,
    pub level: i64,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn all(&self) -> Result<Vec<Item>>;
    fn by_id(&self, id: i64) -> Result<Item>;
    fn by_code(&self, cod: &str) -> Result<Item>;
    fn by_user(&self, user: i64) -> Result<Vec<Item>>;
    fn offsprings(&self, id: i64) -> Result<Vec<Item>>;
    fn users(&self, id: i64) -> Result<Vec<User>>;
    fn create(&self, code: &str, name: &str, parent_id: Option<i64>) -> Result<()>;
    fn update(&self, id: i64, code: &str, name: &str) -> Result<()>;
    fn associate(&self, group: i64, user: i64) -> Result<()>;
    fn dissociate(&self, group: i64, user: i64) -> Result<()>;
    fn destory(&self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self) -> Result<Vec<Item>> {
        let items = groups::dsl::groups
            .order(groups::dsl::name.asc())
            .load(self)?;
        Ok(items)
    }
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = groups::dsl::groups
            .filter(groups::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_code(&self, code: &str) -> Result<Item> {
        let it = groups::dsl::groups
            .filter(groups::dsl::code.eq(code))
            .first(self)?;
        Ok(it)
    }
    fn by_user(&self, user: i64) -> Result<Vec<Item>> {
        let ids: Vec<i64> = groups_users::dsl::groups_users
            .select(groups_users::dsl::group_id)
            .filter(groups_users::dsl::user_id.eq(user))
            .order(groups_users::dsl::created_at.desc())
            .load(self)?;
        let mut items = Vec::new();
        for id in ids {
            let it = Dao::by_id(self, id)?;
            items.push(it);
        }
        Ok(items)
    }
    fn offsprings(&self, id: i64) -> Result<Vec<Item>> {
        let mut children = Vec::new();

        for it in groups::dsl::groups
            .filter(groups::dsl::parent_id.eq(id))
            .load::<Item>(self)?
        {
            children.extend(Dao::offsprings(self, it.id)?);
            children.push(it);
        }
        Ok(children)
    }
    fn users(&self, id: i64) -> Result<Vec<User>> {
        let ids: Vec<i64> = groups_users::dsl::groups_users
            .select(groups_users::dsl::user_id)
            .filter(groups_users::dsl::group_id.eq(id))
            .order(groups_users::dsl::created_at.desc())
            .load(self)?;
        let mut items = Vec::new();
        for id in ids {
            let it = UserDao::by_id(self, id)?;
            items.push(it);
        }
        Ok(items)
    }
    fn create(&self, code: &str, name: &str, parent: Option<i64>) -> Result<()> {
        let now = Utc::now().naive_local();
        let level = match parent {
            Some(id) => {
                let it = Dao::by_id(self, id)?;
                it.id + 1
            }
            None => 1,
        };
        insert_into(groups::dsl::groups)
            .values((
                groups::dsl::code.eq(code),
                groups::dsl::name.eq(name),
                groups::dsl::parent_id.eq(parent),
                groups::dsl::level.eq(level),
                groups::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&self, id: i64, code: &str, name: &str) -> Result<()> {
        let now = Utc::now().naive_local();
        update(groups::dsl::groups.filter(groups::dsl::id.eq(&id)))
            .set((
                groups::dsl::code.eq(code),
                groups::dsl::name.eq(name),
                groups::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn associate(&self, group: i64, user: i64) -> Result<()> {
        let c: i64 = groups_users::dsl::groups_users
            .filter(groups_users::dsl::group_id.eq(group))
            .filter(groups_users::dsl::user_id.eq(user))
            .count()
            .get_result(self)?;
        if c == 0 {
            insert_into(groups_users::dsl::groups_users)
                .values((
                    groups_users::dsl::group_id.eq(group),
                    groups_users::dsl::user_id.eq(user),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn dissociate(&self, group: i64, user: i64) -> Result<()> {
        delete(
            groups_users::dsl::groups_users
                .filter(groups_users::dsl::group_id.eq(group))
                .filter(groups_users::dsl::user_id.eq(user)),
        )
        .execute(self)?;
        Ok(())
    }
    fn destory(&self, id: i64) -> Result<()> {
        let now = Utc::now().naive_local();
        update(groups::dsl::groups.filter(groups::dsl::parent_id.eq(id)))
            .set((
                groups::dsl::parent_id.eq(None::<i64>),
                groups::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        delete(roles_groups::dsl::roles_groups.filter(roles_groups::dsl::group_id.eq(id)))
            .execute(self)?;
        delete(groups_users::dsl::groups_users.filter(groups_users::dsl::group_id.eq(id)))
            .execute(self)?;
        delete(groups::dsl::groups.filter(groups::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
