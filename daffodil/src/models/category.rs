use std::any::type_name;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::Serialize;

use super::super::schema::{categories, category_resources};

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub code: String,
    pub left: i32,
    pub right: i32,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub const ROOT: &str = "root";
}

pub trait Dao {
    fn append(&mut self, code: &str, near: i32) -> Result<()>;
    fn create(&mut self, code: &str, parent: i32) -> Result<()>;
    fn retrieving_full_tree(&mut self, id: i32) -> Result<Vec<Item>>;
    fn retrieving_single_path(&mut self, id: i32) -> Result<Vec<Item>>;
    fn finding_leaf_nodes(&mut self) -> Result<Vec<Item>>;

    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_code(&mut self, code: &str) -> Result<Item>;
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
    fn append(&mut self, code: &str, near: i32) -> Result<()> {
        let it = categories::dsl::categories
            .filter(categories::dsl::id.eq(near))
            .first::<Item>(self)?;
        let now = Utc::now().naive_utc();

        update(categories::dsl::categories.filter(categories::dsl::right.gt(it.right)))
            .set((
                categories::dsl::right.eq(categories::dsl::right + 2),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        update(categories::dsl::categories.filter(categories::dsl::left.gt(it.right)))
            .set((
                categories::dsl::left.eq(categories::dsl::left + 2),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        insert_into(categories::dsl::categories)
            .values((
                categories::dsl::code.eq(code),
                categories::dsl::left.eq(it.right + 1),
                categories::dsl::right.eq(it.right + 2),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn create(&mut self, code: &str, parent: i32) -> Result<()> {
        let it = categories::dsl::categories
            .filter(categories::dsl::id.eq(parent))
            .first::<Item>(self)?;
        let now = Utc::now().naive_utc();

        update(categories::dsl::categories.filter(categories::dsl::right.gt(it.left)))
            .set((
                categories::dsl::right.eq(categories::dsl::right + 2),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        update(categories::dsl::categories.filter(categories::dsl::left.gt(it.left)))
            .set((
                categories::dsl::left.eq(categories::dsl::left + 2),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        insert_into(categories::dsl::categories)
            .values((
                categories::dsl::code.eq(code),
                categories::dsl::left.eq(it.left + 1),
                categories::dsl::right.eq(it.left + 2),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn retrieving_full_tree(&mut self, id: i32) -> Result<Vec<Item>> {
        let it = categories::dsl::categories
            .filter(categories::dsl::id.eq(id))
            .first::<Item>(self)?;
        let items = categories::dsl::categories
            .filter(categories::dsl::left.between(it.left, it.right))
            .order(categories::dsl::left.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn retrieving_single_path(&mut self, id: i32) -> Result<Vec<Item>> {
        let it = categories::dsl::categories
            .filter(categories::dsl::id.eq(id))
            .first::<Item>(self)?;
        let items = categories::dsl::categories
            .filter(categories::dsl::left.le(it.left))
            .filter(categories::dsl::right.ge(it.left))
            .order(categories::dsl::left.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn finding_leaf_nodes(&mut self) -> Result<Vec<Item>> {
        let items = categories::dsl::categories
            .filter(categories::dsl::right.eq(categories::dsl::left + 1))
            .order(categories::dsl::left.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn destroy(&mut self, id: i32) -> Result<()> {
        {
            let it = categories::dsl::categories
                .filter(categories::dsl::id.eq(id))
                .first::<Item>(self)?;
            let width = it.right - it.left + 1;
            let now = Utc::now().naive_utc();
            delete(
                categories::dsl::categories
                    .filter(categories::dsl::left.between(it.left, it.right)),
            )
            .execute(self)?;
            update(categories::dsl::categories.filter(categories::dsl::right.gt(it.right)))
                .set((
                    categories::dsl::right.eq(categories::dsl::right - width),
                    categories::dsl::updated_at.eq(&now),
                ))
                .execute(self)?;
            update(categories::dsl::categories.filter(categories::dsl::left.gt(it.right)))
                .set((
                    categories::dsl::left.eq(categories::dsl::left - width),
                    categories::dsl::updated_at.eq(&now),
                ))
                .execute(self)?;
        }
        delete(
            category_resources::dsl::category_resources
                .filter(category_resources::dsl::category_id.eq(id)),
        )
        .execute(self)?;
        Ok(())
    }

    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = categories::dsl::categories
            .filter(categories::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_code(&mut self, code: &str) -> Result<Item> {
        let it = categories::dsl::categories
            .filter(categories::dsl::code.eq(code))
            .first::<Item>(self)?;
        Ok(it)
    }

    fn set_code(&mut self, id: i32, code: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(categories::dsl::categories.filter(categories::dsl::id.eq(id)))
            .set((
                categories::dsl::code.eq(code),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn index(&mut self) -> Result<Vec<Item>> {
        let items = categories::dsl::categories
            .order(categories::dsl::updated_at.desc())
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
        let ids: Vec<i32> = category_resources::dsl::category_resources
            .select(category_resources::dsl::category_id)
            .filter(category_resources::dsl::resource_type.eq(resource_type))
            .filter(category_resources::dsl::resource_id.eq(resource_id))
            .distinct()
            .order(category_resources::dsl::created_at.desc())
            .load(self)?;
        let items = categories::dsl::categories
            .filter(categories::dsl::id.eq_any(ids))
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_resource_type_(&mut self, resource_type: &str) -> Result<Vec<Item>> {
        let ids: Vec<i32> = category_resources::dsl::category_resources
            .select(category_resources::dsl::category_id)
            .filter(category_resources::dsl::resource_type.eq(resource_type))
            .distinct()
            .order(category_resources::dsl::created_at.desc())
            .load(self)?;
        let items = categories::dsl::categories
            .filter(categories::dsl::id.eq_any(ids))
            .load::<Item>(self)?;
        Ok(items)
    }

    fn associate<T>(&mut self, id: i32, resource_id: i32) -> Result<()> {
        self.associate_(id, type_name::<T>(), resource_id)
    }
    fn dissociate<T>(&mut self, id: i32, resource_id: i32) -> Result<()> {
        self.dissociate_(id, type_name::<T>(), resource_id)
    }
    fn associate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()> {
        let cnt: i64 = category_resources::dsl::category_resources
            .filter(category_resources::dsl::category_id.eq(id))
            .filter(category_resources::dsl::resource_type.eq(resource_type))
            .filter(category_resources::dsl::resource_id.eq(resource_id))
            .count()
            .get_result(self)?;
        if cnt == 0 {
            insert_into(category_resources::dsl::category_resources)
                .values((
                    category_resources::dsl::category_id.eq(id),
                    category_resources::dsl::resource_type.eq(resource_type),
                    category_resources::dsl::resource_id.eq(resource_id),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn dissociate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()> {
        delete(
            category_resources::dsl::category_resources
                .filter(category_resources::dsl::category_id.eq(id))
                .filter(category_resources::dsl::resource_type.eq(resource_type))
                .filter(category_resources::dsl::resource_id.eq(resource_id)),
        )
        .execute(self)?;
        Ok(())
    }
}
