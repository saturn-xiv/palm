use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::schema::menu_items;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub lang: String,
    pub location: String,
    pub label: String,
    pub sort_order: i32,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn index(&mut self) -> Result<Vec<Item>>;
    fn children(&mut self, parent: i32) -> Result<Vec<Item>>;
    fn by_lang_and_location(&mut self, lang: &str, location: &str) -> Result<Vec<Item>>;
    fn root(&mut self, lang: &str, location: &str) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn append(&mut self, lang: &str, location: &str, label: &str, sort_order: i32) -> Result<()>;
    fn create(&mut self, parent: i32, label: &str, sort_order: i32) -> Result<()>;
    fn update(&mut self, id: i32, label: &str, sort_order: i32) -> Result<()>;
    fn destroy(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn index(&mut self) -> Result<Vec<Item>> {
        let items = menu_items::dsl::menu_items
            .order(menu_items::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn children(&mut self, parent: i32) -> Result<Vec<Item>> {
        let items = menu_items::dsl::menu_items
            .filter(menu_items::dsl::parent_id.eq(parent))
            .order(menu_items::dsl::sort_order.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_lang_and_location(&mut self, lang: &str, location: &str) -> Result<Vec<Item>> {
        let items = menu_items::dsl::menu_items
            .filter(menu_items::dsl::lang.eq(lang))
            .filter(menu_items::dsl::location.eq(location))
            .order(menu_items::dsl::sort_order.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn root(&mut self, lang: &str, location: &str) -> Result<Vec<Item>> {
        let items = menu_items::dsl::menu_items
            .filter(menu_items::dsl::lang.eq(lang))
            .filter(menu_items::dsl::location.eq(location))
            .filter(menu_items::dsl::parent_id.is_null())
            .order(menu_items::dsl::sort_order.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = menu_items::dsl::menu_items
            .filter(menu_items::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn append(&mut self, lang: &str, location: &str, label: &str, sort_order: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(menu_items::dsl::menu_items)
            .values((
                menu_items::dsl::lang.eq(lang),
                menu_items::dsl::location.eq(location),
                menu_items::dsl::label.eq(label),
                menu_items::dsl::sort_order.eq(sort_order),
                menu_items::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn create(&mut self, parent: i32, label: &str, sort_order: i32) -> Result<()> {
        let parent = menu_items::dsl::menu_items
            .filter(menu_items::dsl::id.eq(parent))
            .first::<Item>(self)?;
        let now = Utc::now().naive_utc();
        insert_into(menu_items::dsl::menu_items)
            .values((
                menu_items::dsl::parent_id.eq(parent.id),
                menu_items::dsl::lang.eq(&parent.lang),
                menu_items::dsl::location.eq(&parent.location),
                menu_items::dsl::label.eq(label),
                menu_items::dsl::sort_order.eq(sort_order),
                menu_items::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, label: &str, sort_order: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = menu_items::dsl::menu_items.filter(menu_items::dsl::id.eq(id));
        update(it)
            .set((
                menu_items::dsl::label.eq(label),
                menu_items::dsl::sort_order.eq(sort_order),
                menu_items::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn destroy(&mut self, id: i32) -> Result<()> {
        delete(
            menu_items::dsl::menu_items.filter(
                menu_items::dsl::id
                    .eq(id)
                    .or(menu_items::dsl::parent_id.eq(id)),
            ),
        )
        .execute(self)?;
        Ok(())
    }
}
