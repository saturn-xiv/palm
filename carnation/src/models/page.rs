use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Editor, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::cms_pages;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub lang: String,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub body_editor: String,
    pub template: String,
    pub profile: Vec<u8>,
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
    fn count_by_lang(&mut self, lang: &str) -> Result<i64>;
    fn index_by_lang(&mut self, lang: &str, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count_by_user(&mut self, user: i32) -> Result<i64>;
    fn index_by_user(&mut self, user: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_lang_and_slug(&mut self, lang: &str, slug: &str) -> Result<Item>;
    fn create(
        &mut self,
        user: i32,
        lang: &str,
        slug: &str,
        body: &str,
        body_editor: Editor,
        template: &str,
    ) -> Result<()>;
    fn update(&mut self, id: i32, slug: &str, body: &str) -> Result<()>;
    fn set_template(&mut self, id: i32, template: &str) -> Result<()>;
    fn set_profile(&mut self, id: i32, profile: &Profile) -> Result<()>;
    fn lock(&mut self, id: i32) -> Result<()>;
    fn unlock(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn count_by_lang(&mut self, lang: &str) -> Result<i64> {
        let it = cms_pages::dsl::cms_pages
            .filter(cms_pages::dsl::lang.eq(lang))
            .count()
            .first(self)?;
        Ok(it)
    }
    fn index_by_lang(&mut self, lang: &str, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = cms_pages::dsl::cms_pages
            .filter(cms_pages::dsl::lang.eq(lang))
            .order(cms_pages::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn index_by_user(&mut self, user: i32, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = cms_pages::dsl::cms_pages
            .filter(cms_pages::dsl::user_id.eq(user))
            .order(cms_pages::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count_by_user(&mut self, user: i32) -> Result<i64> {
        let it = cms_pages::dsl::cms_pages
            .filter(cms_pages::dsl::user_id.eq(user))
            .count()
            .first(self)?;
        Ok(it)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = cms_pages::dsl::cms_pages
            .filter(cms_pages::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_lang_and_slug(&mut self, lang: &str, slug: &str) -> Result<Item> {
        let it = cms_pages::dsl::cms_pages
            .filter(cms_pages::dsl::lang.eq(lang))
            .filter(cms_pages::dsl::slug.eq(slug))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i32,
        lang: &str,
        slug: &str,
        body: &str,
        editor: Editor,
        template: &str,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(cms_pages::dsl::cms_pages)
            .values((
                cms_pages::user_id.eq(user),
                cms_pages::lang.eq(lang),
                cms_pages::slug.eq(slug),
                cms_pages::body.eq(&body),
                cms_pages::body_editor.eq(&editor.to_string()),
                cms_pages::status.eq(&Status::Pending.to_string()),
                cms_pages::template.eq(template),
                cms_pages::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, slug: &str, body: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = cms_pages::dsl::cms_pages.filter(cms_pages::dsl::id.eq(id));
        update(it)
            .set((
                cms_pages::dsl::slug.eq(slug),
                cms_pages::dsl::body.eq(body),
                cms_pages::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_template(&mut self, id: i32, template: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = cms_pages::dsl::cms_pages.filter(cms_pages::dsl::id.eq(id));
        update(it)
            .set((
                cms_pages::dsl::template.eq(template),
                cms_pages::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_profile(&mut self, id: i32, profile: &Profile) -> Result<()> {
        let profile = flexbuffers::to_vec(profile)?;
        let now = Utc::now().naive_utc();
        let it = cms_pages::dsl::cms_pages.filter(cms_pages::dsl::id.eq(id));
        update(it)
            .set((
                cms_pages::dsl::profile.eq(&profile),
                cms_pages::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn lock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = cms_pages::dsl::cms_pages.filter(cms_pages::dsl::id.eq(id));
        update(it)
            .set((
                cms_pages::dsl::locked_at.eq(&Some(now)),
                cms_pages::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = cms_pages::dsl::cms_pages.filter(cms_pages::dsl::id.eq(id));
        update(it)
            .set((
                cms_pages::dsl::locked_at.eq(&None::<NaiveDateTime>),
                cms_pages::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = cms_pages::dsl::cms_pages.filter(cms_pages::dsl::id.eq(id));
        update(it)
            .set((
                cms_pages::dsl::deleted_at.eq(&Some(now)),
                cms_pages::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = cms_pages::dsl::cms_pages.filter(cms_pages::dsl::id.eq(id));
        update(it)
            .set((
                cms_pages::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                cms_pages::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
