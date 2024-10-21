use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::schema::questionnaire_polls;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Value {}

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub form_id: i32,
    pub batch_no: String,
    pub field_id: i32,
    pub value: Vec<u8>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_form(&mut self, form: i32) -> Result<Vec<Item>>;
    fn by_batch_no(&mut self, batch_no: &str) -> Result<Vec<Item>>;
    fn by_field(&mut self, field: i32) -> Result<Vec<Item>>;
    fn create(&mut self, form: i32, field: i32, batch_no: &str, value: &Value) -> Result<()>;
    fn update(&mut self, id: i32, value: &Value) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}
impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = questionnaire_polls::dsl::questionnaire_polls
            .filter(questionnaire_polls::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_form(&mut self, form: i32) -> Result<Vec<Item>> {
        let items = questionnaire_polls::dsl::questionnaire_polls
            .filter(questionnaire_polls::dsl::form_id.eq(form))
            .order(questionnaire_polls::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_batch_no(&mut self, batch_no: &str) -> Result<Vec<Item>> {
        let items = questionnaire_polls::dsl::questionnaire_polls
            .filter(questionnaire_polls::dsl::batch_no.eq(batch_no))
            .order(questionnaire_polls::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_field(&mut self, field: i32) -> Result<Vec<Item>> {
        let items = questionnaire_polls::dsl::questionnaire_polls
            .filter(questionnaire_polls::dsl::field_id.eq(field))
            .order(questionnaire_polls::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn create(&mut self, form: i32, field: i32, batch_no: &str, value: &Value) -> Result<()> {
        let now = Utc::now().naive_utc();
        let value = flexbuffers::to_vec(value)?;
        insert_into(questionnaire_polls::dsl::questionnaire_polls)
            .values((
                questionnaire_polls::form_id.eq(form),
                questionnaire_polls::field_id.eq(field),
                questionnaire_polls::batch_no.eq(batch_no),
                questionnaire_polls::value.eq(&value),
                questionnaire_polls::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, value: &Value) -> Result<()> {
        let now = Utc::now().naive_utc();
        let value = flexbuffers::to_vec(value)?;
        let it = questionnaire_polls::dsl::questionnaire_polls
            .filter(questionnaire_polls::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_polls::dsl::value.eq(&value),
                questionnaire_polls::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_polls::dsl::questionnaire_polls
            .filter(questionnaire_polls::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_polls::dsl::deleted_at.eq(&Some(now)),
                questionnaire_polls::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_polls::dsl::questionnaire_polls
            .filter(questionnaire_polls::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_polls::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                questionnaire_polls::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
