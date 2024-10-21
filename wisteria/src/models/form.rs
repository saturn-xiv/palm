use std::string::ToString;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Editor, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use uuid::Uuid;

use super::super::schema::questionnaire_forms;

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    #[default]
    Pending,
    Opening,
    Closed,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {}

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub uid: String,
    pub title: String,
    pub description: String,
    pub description_editor: String,
    pub profile: Vec<u8>,
    pub status: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>>;
    fn create(&mut self, user: i32, title: &str, description: &str, editor: Editor) -> Result<()>;
    fn update(&mut self, id: i32, title: &str, description: &str) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn lock(&mut self, id: i32) -> Result<()>;
    fn unlock(&mut self, id: i32) -> Result<()>;
    fn set_status(&mut self, id: i32, status: Status) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::uid.eq(uid))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>> {
        let items = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::user_id.eq(user))
            .order(questionnaire_forms::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn create(&mut self, user: i32, title: &str, description: &str, editor: Editor) -> Result<()> {
        let uid = Uuid::new_v4().to_string();
        let now = Utc::now().naive_utc();
        let profile = flexbuffers::to_vec(Profile::default())?;
        insert_into(questionnaire_forms::dsl::questionnaire_forms)
            .values((
                questionnaire_forms::user_id.eq(user),
                questionnaire_forms::title.eq(title),
                questionnaire_forms::uid.eq(&uid),
                questionnaire_forms::description.eq(description),
                questionnaire_forms::description_editor.eq(&editor.to_string()),
                questionnaire_forms::status.eq(&Status::Pending.to_string()),
                questionnaire_forms::profile.eq(&profile),
                questionnaire_forms::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, title: &str, description: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_forms::dsl::title.eq(title),
                questionnaire_forms::dsl::description.eq(description),
                questionnaire_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_forms::dsl::deleted_at.eq(&Some(now)),
                questionnaire_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_forms::dsl::deleted_at.eq(None::<NaiveDateTime>),
                questionnaire_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn lock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_forms::dsl::locked_at.eq(&Some(now)),
                questionnaire_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_forms::dsl::locked_at.eq(&None::<NaiveDateTime>),
                questionnaire_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_status(&mut self, id: i32, status: Status) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = questionnaire_forms::dsl::questionnaire_forms
            .filter(questionnaire_forms::dsl::id.eq(id));
        update(it)
            .set((
                questionnaire_forms::dsl::status.eq(&status.to_string()),
                questionnaire_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
