use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::bbs_forums;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub lang: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub profile: Vec<u8>,
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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Profile {}

impl Profile {
    pub fn new(buf: &[u8]) -> Result<Self> {
        let it = flexbuffers::from_slice(buf)?;
        Ok(it)
    }
}

pub trait Dao {
    fn by_lang(&mut self, lang: &str) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_slug(&mut self, slug: &str) -> Result<Item>;
    fn create(&mut self, lang: &str, slug: &str, title: &str, description: &str) -> Result<()>;
    fn update(&mut self, id: i32, slug: &str, body: &str, description: &str) -> Result<()>;
    fn set_profile(&mut self, id: i32, profile: &Profile) -> Result<()>;
    fn lock(&mut self, id: i32) -> Result<()>;
    fn unlock(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_lang(&mut self, lang: &str) -> Result<Vec<Item>> {
        let items = bbs_forums::dsl::bbs_forums
            .filter(bbs_forums::dsl::lang.eq(lang))
            .order(bbs_forums::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bbs_forums::dsl::bbs_forums
            .filter(bbs_forums::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_slug(&mut self, slug: &str) -> Result<Item> {
        let it = bbs_forums::dsl::bbs_forums
            .filter(bbs_forums::dsl::slug.eq(slug))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&mut self, lang: &str, slug: &str, title: &str, description: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let profile = flexbuffers::to_vec(Profile::default())?;
        insert_into(bbs_forums::dsl::bbs_forums)
            .values((
                bbs_forums::lang.eq(lang),
                bbs_forums::slug.eq(slug),
                bbs_forums::title.eq(title),
                bbs_forums::description.eq(&description),
                bbs_forums::status.eq(&Status::Pending.to_string()),
                bbs_forums::profile.eq(&profile),
                bbs_forums::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, slug: &str, title: &str, description: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_forums::dsl::bbs_forums.filter(bbs_forums::dsl::id.eq(id));
        update(it)
            .set((
                bbs_forums::dsl::slug.eq(slug),
                bbs_forums::dsl::title.eq(title),
                bbs_forums::dsl::description.eq(description),
                bbs_forums::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_profile(&mut self, id: i32, profile: &Profile) -> Result<()> {
        let profile = flexbuffers::to_vec(profile)?;
        let now = Utc::now().naive_utc();
        let it = bbs_forums::dsl::bbs_forums.filter(bbs_forums::dsl::id.eq(id));
        update(it)
            .set((
                bbs_forums::dsl::profile.eq(&profile),
                bbs_forums::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn lock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_forums::dsl::bbs_forums.filter(bbs_forums::dsl::id.eq(id));
        update(it)
            .set((
                bbs_forums::dsl::locked_at.eq(&Some(now)),
                bbs_forums::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_forums::dsl::bbs_forums.filter(bbs_forums::dsl::id.eq(id));
        update(it)
            .set((
                bbs_forums::dsl::locked_at.eq(&None::<NaiveDateTime>),
                bbs_forums::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_forums::dsl::bbs_forums.filter(bbs_forums::dsl::id.eq(id));
        update(it)
            .set((
                bbs_forums::dsl::deleted_at.eq(&Some(now)),
                bbs_forums::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bbs_forums::dsl::bbs_forums.filter(bbs_forums::dsl::id.eq(id));
        update(it)
            .set((
                bbs_forums::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                bbs_forums::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
