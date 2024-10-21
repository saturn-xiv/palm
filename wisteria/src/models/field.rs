use std::string::ToString;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::questionnaire_fields;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {}

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub form_id: i32,
    pub uid: String,
    pub label: String,
    pub summary: String,
    pub profile: Vec<u8>,
    pub sort_order: i32,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>>;
    fn by_form(&mut self, form: i32) -> Result<Vec<Item>>;
    fn create(
        &mut self,
        user: i32,
        form: i32,
        label: &str,
        summary: &str,
        sort_order: i32,
    ) -> Result<()>;
    fn update(&mut self, id: i32, label: &str, summary: &str, sort_order: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = questionnaire_fields::dsl::questionnaire_fields
            .filter(questionnaire_fields::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = questionnaire_fields::dsl::questionnaire_fields
            .filter(questionnaire_fields::dsl::uid.eq(uid))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>> {
        let items = questionnaire_fields::dsl::questionnaire_fields
            .filter(questionnaire_fields::dsl::user_id.eq(user))
            .order(questionnaire_fields::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_form(&mut self, form: i32) -> Result<Vec<Item>> {
        let items = questionnaire_fields::dsl::questionnaire_fields
            .filter(questionnaire_fields::dsl::form_id.eq(form))
            .order(questionnaire_fields::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn create(
        &mut self,
        user: i32,
        form: i32,
        label: &str,
        summary: &str,
        sort_order: i32,
    ) -> Result<()> {
        let uid = Uuid::new_v4().to_string();
        let now = Utc::now().naive_utc();
        let profile = flexbuffers::to_vec(Profile::default())?;
        insert_into(questionnaire_fields::dsl::questionnaire_fields)
            .values((
                questionnaire_fields::user_id.eq(user),
                questionnaire_fields::form_id.eq(form),
                questionnaire_fields::label.eq(label),
                questionnaire_fields::uid.eq(&uid),
                questionnaire_fields::summary.eq(summary),
                questionnaire_fields::sort_order.eq(sort_order),
                questionnaire_fields::profile.eq(&profile),
                questionnaire_fields::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, label: &str, summary: &str, sort_order: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_fields::dsl::questionnaire_fields
            .filter(questionnaire_fields::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_fields::dsl::label.eq(label),
                questionnaire_fields::dsl::summary.eq(summary),
                questionnaire_fields::dsl::sort_order.eq(sort_order),
                questionnaire_fields::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_fields::dsl::questionnaire_fields
            .filter(questionnaire_fields::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_fields::dsl::deleted_at.eq(&Some(now)),
                questionnaire_fields::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_fields::dsl::questionnaire_fields
            .filter(questionnaire_fields::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_fields::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                questionnaire_fields::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
