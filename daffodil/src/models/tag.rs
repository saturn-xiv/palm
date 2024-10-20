use std::any::type_name;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::Serialize;

use super::super::schema::{tag_resources, tags};

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub code: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_code(&mut self, code: &str) -> Result<Item>;
    fn create(&mut self, code: &str) -> Result<()>;
    fn set_code(&mut self, id: i32, code: &str) -> Result<()>;
    fn index(&mut self) -> Result<Vec<Item>>;
    fn by_resource<T>(&mut self, resource_id: i32) -> Result<Vec<Item>>;
    fn by_resource_type<T>(&mut self) -> Result<Vec<Item>>;
    fn by_resource_(&mut self, resource_type: &str, resource_id: i32) -> Result<Vec<Item>>;
    fn by_resource_type_(&mut self, resource_type: &str) -> Result<Vec<Item>>;

    fn destroy(&mut self, id: i32) -> Result<()>;

    fn associate<T>(&mut self, id: i32, resource_id: i32) -> Result<()>;
    fn dissociate<T>(&mut self, id: i32, resource_id: i32) -> Result<()>;
    fn associate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()>;
    fn dissociate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = tags::dsl::tags
            .filter(tags::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_code(&mut self, code: &str) -> Result<Item> {
        let it = tags::dsl::tags
            .filter(tags::dsl::code.eq(code))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&mut self, code: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(tags::dsl::tags)
            .values((tags::dsl::code.eq(code), tags::dsl::updated_at.eq(&now)))
            .execute(self)?;
        Ok(())
    }

    fn set_code(&mut self, id: i32, code: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(tags::dsl::tags.filter(tags::dsl::id.eq(id)))
            .set((tags::dsl::code.eq(code), tags::dsl::updated_at.eq(&now)))
            .execute(self)?;
        Ok(())
    }

    fn index(&mut self) -> Result<Vec<Item>> {
        let items = tags::dsl::tags
            .order(tags::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn by_resource<T>(&mut self, resource_id: i32) -> Result<Vec<Item>> {
        self.by_resource_(type_name::<T>(), resource_id)
    }
    fn by_resource_type<T>(&mut self) -> Result<Vec<Item>> {
        self.by_resource_type_(type_name::<T>())
    }
    fn by_resource_(&mut self, resource_type: &str, resource_id: i32) -> Result<Vec<Item>> {
        let ids: Vec<i32> = tag_resources::dsl::tag_resources
            .select(tag_resources::dsl::tag_id)
            .filter(tag_resources::dsl::resource_type.eq(resource_type))
            .filter(tag_resources::dsl::resource_id.eq(resource_id))
            .distinct()
            .order(tag_resources::dsl::created_at.desc())
            .load(self)?;
        let items = tags::dsl::tags
            .filter(tags::dsl::id.eq_any(ids))
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_resource_type_(&mut self, resource_type: &str) -> Result<Vec<Item>> {
        let ids: Vec<i32> = tag_resources::dsl::tag_resources
            .select(tag_resources::dsl::tag_id)
            .filter(tag_resources::dsl::resource_type.eq(resource_type))
            .distinct()
            .order(tag_resources::dsl::created_at.desc())
            .load(self)?;
        let items = tags::dsl::tags
            .filter(tags::dsl::id.eq_any(ids))
            .load::<Item>(self)?;
        Ok(items)
    }

    fn destroy(&mut self, id: i32) -> Result<()> {
        delete(tag_resources::dsl::tag_resources.filter(tag_resources::dsl::tag_id.eq(id)))
            .execute(self)?;
        delete(tags::dsl::tags.filter(tags::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
    fn associate<T>(&mut self, id: i32, resource_id: i32) -> Result<()> {
        self.associate_(id, type_name::<T>(), resource_id)
    }
    fn dissociate<T>(&mut self, id: i32, resource_id: i32) -> Result<()> {
        self.dissociate_(id, type_name::<T>(), resource_id)
    }
    fn associate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()> {
        let cnt: i64 = tag_resources::dsl::tag_resources
            .filter(tag_resources::dsl::tag_id.eq(id))
            .filter(tag_resources::dsl::resource_type.eq(resource_type))
            .filter(tag_resources::dsl::resource_id.eq(resource_id))
            .count()
            .get_result(self)?;
        if cnt == 0 {
            insert_into(tag_resources::dsl::tag_resources)
                .values((
                    tag_resources::dsl::tag_id.eq(id),
                    tag_resources::dsl::resource_type.eq(resource_type),
                    tag_resources::dsl::resource_id.eq(resource_id),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn dissociate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()> {
        delete(
            tag_resources::dsl::tag_resources
                .filter(tag_resources::dsl::tag_id.eq(id))
                .filter(tag_resources::dsl::resource_type.eq(resource_type))
                .filter(tag_resources::dsl::resource_id.eq(resource_id)),
        )
        .execute(self)?;
        Ok(())
    }
}
