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
        let buf = Dao::get(self, enc, &type_name::<V>().to_string(), user).await?;
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
        let buf = flexbuffers::to_vec(value)?;
        Dao::set(self, enc, &type_name::<V>().to_string(), user, &buf, encode).await?;
        Ok(())
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
    fn get<K: Display, E: SecretBox>(
        &mut self,
        e: &E,
        k: &K,
        u: Option<i64>,
    ) -> impl Future<Output = Result<Vec<u8>>>;
    fn set<K: Display, E: SecretBox>(
        &mut self,
        e: &E,
        k: &K,
        u: Option<i64>,
        v: &[u8],
        f: bool,
    ) -> impl Future<Output = Result<()>>;
    fn delete(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    async fn get<K: Display, E: SecretBox>(
        &mut self,
        e: &E,
        k: &K,
        u: Option<i64>,
    ) -> Result<Vec<u8>> {
        let k = k.to_string();

        let it = match u {
            Some(ref u) => settings::dsl::settings
                .filter(settings::dsl::key.eq(&k))
                .filter(settings::dsl::user_id.eq(u))
                .first::<Item>(self)?,
            None => settings::dsl::settings
                .filter(settings::dsl::key.eq(&k))
                .filter(settings::dsl::user_id.is_null())
                .first::<Item>(self)?,
        };

        let val = match it.associated_data {
            Some(ref associated_data) => e.decrypt(&it.value, associated_data).await?,
            None => it.value,
        };
        Ok(val)
    }

    async fn set<K: Display, E: SecretBox>(
        &mut self,
        e: &E,
        k: &K,
        u: Option<i64>,
        v: &[u8],
        f: bool,
    ) -> Result<()> {
        let k = k.to_string();

        let (val, associated_data) = if f {
            let (val, associated_data) = e.encrypt(v).await?;
            (val, Some(associated_data))
        } else {
            (v.to_vec(), None)
        };

        let now = Utc::now().naive_utc();

        let it = match u {
            Some(ref u) => settings::dsl::settings
                .filter(settings::dsl::key.eq(&k))
                .filter(settings::dsl::user_id.eq(u))
                .first::<Item>(self),
            None => settings::dsl::settings
                .filter(settings::dsl::key.eq(&k))
                .filter(settings::dsl::user_id.is_null())
                .first::<Item>(self),
        };

        match it {
            Ok(it) => {
                let it = settings::dsl::settings.filter(settings::dsl::id.eq(&it.id));

                update(it)
                    .set((
                        settings::dsl::value.eq(&val),
                        settings::dsl::user_id.eq(u),
                        settings::dsl::associated_data.eq(&associated_data),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(settings::dsl::settings)
                    .values((
                        settings::dsl::key.eq(&k),
                        settings::dsl::user_id.eq(u),
                        settings::dsl::value.eq(&val),
                        settings::dsl::associated_data.eq(&associated_data),
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
