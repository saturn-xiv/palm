use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use serde::Serialize;

use super::super::super::super::{orm::postgresql::Connection, Result};
use super::{
    group::{Dao as GroupDao, Item as Group},
    schema::{policies, roles, roles_groups, roles_relations, roles_users},
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

impl Item {
    pub const ADMINISTRATOR: &'static str = "administrator";
    pub const ROOT: &'static str = "root";
}

pub trait Dao {
    fn all(&self) -> Result<Vec<Item>>;
    fn by_id(&self, id: i64) -> Result<Item>;
    fn by_code(&self, cod: &str) -> Result<Item>;
    fn by_user(&self, user: i64) -> Result<Vec<Item>>;
    fn by_group(&self, group: i64) -> Result<Vec<Item>>;
    fn offsprings(&self, id: i64) -> Result<Vec<Item>>;
    fn users(&self, id: i64) -> Result<Vec<User>>;
    fn groups(&self, id: i64) -> Result<Vec<Group>>;
    fn create(&self, code: &str, name: &str, parent: Option<i64>) -> Result<()>;
    fn update(&self, id: i64, code: &str, name: &str) -> Result<()>;
    fn associate_user(&self, role: i64, user: i64) -> Result<()>;
    fn dissociate_user(&self, role: i64, user: i64) -> Result<()>;
    fn associate_group(&self, role: i64, group: i64) -> Result<()>;
    fn dissociate_group(&self, role: i64, group: i64) -> Result<()>;
    fn destory(&self, id: i64) -> Result<()>;
    fn has_user(&self, role: i64, user: i64) -> Result<bool>;
}

impl Dao for Connection {
    fn all(&self) -> Result<Vec<Item>> {
        let items = roles::dsl::roles.order(roles::dsl::name.asc()).load(self)?;
        Ok(items)
    }
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = roles::dsl::roles
            .filter(roles::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_code(&self, code: &str) -> Result<Item> {
        let it = roles::dsl::roles
            .filter(roles::dsl::code.eq(code))
            .first(self)?;
        Ok(it)
    }
    fn by_user(&self, user: i64) -> Result<Vec<Item>> {
        let ids: Vec<i64> = roles_users::dsl::roles_users
            .select(roles_users::dsl::role_id)
            .filter(roles_users::dsl::user_id.eq(user))
            .order(roles_users::dsl::created_at.desc())
            .load(self)?;
        let mut items = Vec::new();
        for id in ids {
            let it = Dao::by_id(self, id)?;
            items.push(it);
        }
        Ok(items)
    }
    fn by_group(&self, group: i64) -> Result<Vec<Item>> {
        let ids: Vec<i64> = roles_groups::dsl::roles_groups
            .select(roles_groups::dsl::role_id)
            .filter(roles_groups::dsl::group_id.eq(group))
            .order(roles_groups::dsl::created_at.desc())
            .load(self)?;
        let mut items = Vec::new();
        for id in ids {
            let it = Dao::by_id(self, id)?;
            items.push(it);
        }
        Ok(items)
    }
    fn users(&self, id: i64) -> Result<Vec<User>> {
        let ids: Vec<i64> = roles_users::dsl::roles_users
            .select(roles_users::dsl::user_id)
            .filter(roles_users::dsl::role_id.eq(id))
            .order(roles_users::dsl::created_at.desc())
            .load(self)?;
        let mut items = Vec::new();
        for id in ids {
            let it = UserDao::by_id(self, id)?;
            items.push(it);
        }
        Ok(items)
    }
    fn groups(&self, id: i64) -> Result<Vec<Group>> {
        let ids: Vec<i64> = roles_groups::dsl::roles_groups
            .select(roles_groups::dsl::group_id)
            .filter(roles_groups::dsl::role_id.eq(id))
            .order(roles_groups::dsl::created_at.desc())
            .load(self)?;
        let mut items = Vec::new();
        for id in ids {
            let it = GroupDao::by_id(self, id)?;
            items.push(it);
        }
        Ok(items)
    }
    fn offsprings(&self, id: i64) -> Result<Vec<Item>> {
        let mut children = Vec::new();

        for it in roles::dsl::roles
            .filter(roles::dsl::parent_id.eq(id))
            .load::<Item>(self)?
        {
            children.extend(Dao::offsprings(self, it.id)?);
            children.push(it);
        }
        Ok(children)
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
        insert_into(roles::dsl::roles)
            .values((
                roles::dsl::code.eq(code),
                roles::dsl::name.eq(name),
                roles::dsl::parent_id.eq(parent),
                roles::dsl::level.eq(level),
                roles::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&self, id: i64, code: &str, name: &str) -> Result<()> {
        let now = Utc::now().naive_local();
        update(roles::dsl::roles.filter(roles::dsl::id.eq(&id)))
            .set((
                roles::dsl::code.eq(code),
                roles::dsl::name.eq(name),
                roles::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn associate_user(&self, role: i64, user: i64) -> Result<()> {
        let c: i64 = roles_users::dsl::roles_users
            .filter(roles_users::dsl::role_id.eq(role))
            .filter(roles_users::dsl::user_id.eq(user))
            .count()
            .get_result(self)?;
        if c == 0 {
            insert_into(roles_users::dsl::roles_users)
                .values((
                    roles_users::dsl::role_id.eq(role),
                    roles_users::dsl::user_id.eq(user),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn dissociate_user(&self, role: i64, user: i64) -> Result<()> {
        delete(
            roles_users::dsl::roles_users
                .filter(roles_users::dsl::role_id.eq(role))
                .filter(roles_users::dsl::user_id.eq(user)),
        )
        .execute(self)?;
        Ok(())
    }
    fn associate_group(&self, role: i64, group: i64) -> Result<()> {
        let c: i64 = roles_groups::dsl::roles_groups
            .filter(roles_groups::dsl::role_id.eq(role))
            .filter(roles_groups::dsl::group_id.eq(group))
            .count()
            .get_result(self)?;
        if c == 0 {
            insert_into(roles_groups::dsl::roles_groups)
                .values((
                    roles_groups::dsl::role_id.eq(role),
                    roles_groups::dsl::group_id.eq(group),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn dissociate_group(&self, role: i64, group: i64) -> Result<()> {
        delete(
            roles_groups::dsl::roles_groups
                .filter(roles_groups::dsl::role_id.eq(role))
                .filter(roles_groups::dsl::group_id.eq(group)),
        )
        .execute(self)?;
        Ok(())
    }
    fn destory(&self, id: i64) -> Result<()> {
        let now = Utc::now().naive_local();
        update(roles::dsl::roles.filter(roles::dsl::parent_id.eq(id)))
            .set((
                roles::dsl::parent_id.eq(None::<i64>),
                roles::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        delete(roles_groups::dsl::roles_groups.filter(roles_groups::dsl::role_id.eq(id)))
            .execute(self)?;
        delete(roles_users::dsl::roles_users.filter(roles_users::dsl::role_id.eq(id)))
            .execute(self)?;
        delete(
            roles_relations::dsl::roles_relations.filter(
                roles_relations::dsl::a
                    .eq(id)
                    .or(roles_relations::dsl::b.eq(id)),
            ),
        )
        .execute(self)?;
        delete(policies::dsl::policies.filter(policies::dsl::role_id.eq(id))).execute(self)?;
        delete(roles::dsl::roles.filter(roles::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
    fn has_user(&self, role: i64, user: i64) -> Result<bool> {
        let it: i64 = roles_users::dsl::roles_users
            .filter(roles_users::dsl::role_id.eq(role))
            .filter(roles_users::dsl::user_id.eq(user))
            .count()
            .first(self)?;
        Ok(it > 0)
    }
}
