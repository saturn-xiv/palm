use std::any::type_name;
use std::fmt::Display;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use hyacinth::schema::settings;
use serde::{de::DeserializeOwned, ser::Serialize};

use super::super::{Result, SecretBox, orm::postgresql::Connection};

pub trait FlatBuffer {
    fn get<V: DeserializeOwned, E: SecretBox>(
        &mut self,
        enc: &E,
        user: Option<i64>,
    ) -> impl Future<Output = Result<V>>;
    fn set<V: Serialize, E: SecretBox>(
        &mut self,
        enc: &E,
        user: Option<i64>,
        value: &V,
        encode: bool,
    ) -> impl Future<Output = Result<()>>;
}

impl FlatBuffer for Connection {
    async fn get<V: DeserializeOwned, E: SecretBox>(
        &mut self,
        enc: &E,
        user: Option<i64>,
    ) -> Result<V> {
        let it = Dao::get(self, &type_name::<V>().to_string(), user)?;
        let buf = match it.associated_data {
            Some(ref associated_data) => enc.decrypt(&it.value, associated_data).await?,
            None => it.value,
        };
        let it = flexbuffers::from_slice(&buf)?;
        Ok(it)
    }

    async fn set<V: Serialize, E: SecretBox>(
        &mut self,
        enc: &E,
        user: Option<i64>,
        value: &V,
        encode: bool,
    ) -> Result<()> {
        let key = type_name::<V>().to_string();
        let value = flexbuffers::to_vec(value)?;
        if encode {
            return Dao::set(self, &key, user, &value, None);
        }
        let (value, associated_data) = enc.encrypt(&value).await?;
        Dao::set(self, &key, user, &value, Some(associated_data.as_ref()))
    }
}

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub user_id: Option<i64>,
    pub key: String,
    pub value: Vec<u8>,
    pub associated_data: Option<Vec<u8>>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn get<K: Display>(&mut self, key: &K, user: Option<i64>) -> Result<Item>;
    fn set<K: Display>(
        &mut self,
        key: &K,
        user: Option<i64>,
        value: &[u8],
        associated_data: Option<&[u8]>,
    ) -> Result<()>;
    fn delete(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn get<K: Display>(&mut self, key: &K, user: Option<i64>) -> Result<Item> {
        let key = key.to_string();

        let it = match user {
            Some(ref user) => settings::dsl::settings
                .filter(settings::dsl::key.eq(&key))
                .filter(settings::dsl::user_id.eq(user))
                .first::<Item>(self)?,
            None => settings::dsl::settings
                .filter(settings::dsl::key.eq(&key))
                .filter(settings::dsl::user_id.is_null())
                .first::<Item>(self)?,
        };

        Ok(it)
    }

    fn set<K: Display>(
        &mut self,
        key: &K,
        user: Option<i64>,
        value: &[u8],
        associated_data: Option<&[u8]>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();

        match Dao::get(self, key, user) {
            Ok(it) => {
                let it = settings::dsl::settings.filter(settings::dsl::id.eq(&it.id));

                update(it)
                    .set((
                        settings::dsl::value.eq(value),
                        settings::dsl::associated_data.eq(associated_data),
                        settings::dsl::version.eq(settings::dsl::version + 1),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                let key = key.to_string();
                insert_into(settings::dsl::settings)
                    .values((
                        settings::dsl::key.eq(&key),
                        settings::dsl::user_id.eq(user),
                        settings::dsl::value.eq(value),
                        settings::dsl::associated_data.eq(associated_data),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
        };
        Ok(())
    }

    fn delete(&mut self, id: i64) -> Result<()> {
        delete(settings::dsl::settings.filter(settings::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
