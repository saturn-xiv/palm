use std::any::type_name;
use std::fmt::Display;

use aes::Aes256;
use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256GcmSiv, Key, Nonce,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use hyper::StatusCode;
use petunia::{orm::postgresql::Connection, HttpError, Result};
use serde::{de::DeserializeOwned, ser::Serialize};

use super::super::schema::settings;

pub trait Protobuf {
    fn get<V: prost::Message + Default>(&mut self, user: Option<i32>) -> Result<V>;
    fn set<V: prost::Message + Default>(
        &mut self,
        user: Option<i32>,
        value: &V,
        encode: bool,
    ) -> Result<()>;
}

impl<'a> Protobuf for Setting<'a> {
    fn get<V: prost::Message + Default>(&mut self, user: Option<i32>) -> Result<V> {
        let buf: Vec<u8> = Setting::get(self, &type_name::<V>().to_string(), user)?;
        let it = V::decode(&buf[..])?;
        Ok(it)
    }

    fn set<V: prost::Message + Default>(
        &mut self,
        user: Option<i32>,
        value: &V,
        encrypt: bool,
    ) -> Result<()> {
        let mut buf = Vec::new();
        value.encode(&mut buf)?;
        Setting::set(self, &type_name::<V>().to_string(), user, &buf, encrypt)?;
        Ok(())
    }
}

pub trait FlatBuffer {
    fn get<V: DeserializeOwned>(&mut self, user: Option<i32>) -> Result<V>;
    fn set<V: Serialize>(&mut self, user: Option<i32>, value: &V, encrypt: bool) -> Result<()>;
}

impl<'a> FlatBuffer for Setting<'a> {
    fn get<V: DeserializeOwned>(&mut self, user: Option<i32>) -> Result<V> {
        let buf: Vec<u8> = Setting::get(self, &type_name::<V>().to_string(), user)?;
        let it = flexbuffers::from_slice(&buf)?;
        Ok(it)
    }

    fn set<V: Serialize>(&mut self, user: Option<i32>, value: &V, encrypt: bool) -> Result<()> {
        let buf = flexbuffers::to_vec(value)?;
        Setting::set(self, &type_name::<V>().to_string(), user, &buf, encrypt)?;
        Ok(())
    }
}
pub struct Setting<'a> {
    pub db: &'a mut Connection,
    pub cipher: Aes256GcmSiv,
}

impl<'a> Setting<'a> {
    pub fn new(key: &[u8], db: &'a mut Connection) -> Self {
        let key = Key::<Aes256>::from_slice(key);
        Self {
            cipher: Aes256GcmSiv::new(key),
            db,
        }
    }
    fn get<K: Display>(&mut self, key: &K, user: Option<i32>) -> Result<Vec<u8>> {
        let (value, nonce) = self.db.get(key, user)?;
        match nonce {
            Some(ref nonce) => {
                let nonce = Nonce::from_slice(nonce);
                let value = self.cipher.decrypt(nonce, value.as_ref()).map_err(|x| {
                    Box::new(HttpError(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Some(x.to_string()),
                    ))
                })?;
                Ok(value)
            }
            None => Ok(value),
        }
    }
    fn set<K: Display>(
        &mut self,
        key: &K,
        user: Option<i32>,
        value: &[u8],
        encrypt: bool,
    ) -> Result<()> {
        if !encrypt {
            return self.db.set(key, user, value, None);
        }
        let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);
        let value = self.cipher.encrypt(&nonce, value).map_err(|x| {
            Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(x.to_string()),
            ))
        })?;
        self.db.set(key, user, &value, Some(nonce.as_slice()))?;

        Ok(())
    }
}

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub user_id: Option<i32>,
    pub key: String,
    pub value: Vec<u8>,
    pub nonce: Option<Vec<u8>>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn get<K: Display>(&mut self, key: &K, user: Option<i32>)
        -> Result<(Vec<u8>, Option<Vec<u8>>)>;
    fn set<K: Display>(
        &mut self,
        key: &K,
        user: Option<i32>,
        value: &[u8],
        nonce: Option<&[u8]>,
    ) -> Result<()>;
}

impl Dao for Connection {
    fn get<K: Display>(
        &mut self,
        key: &K,
        user: Option<i32>,
    ) -> Result<(Vec<u8>, Option<Vec<u8>>)> {
        let key = key.to_string();

        let it = match user {
            Some(user) => settings::dsl::settings
                .filter(settings::dsl::key.eq(&key))
                .filter(settings::dsl::user_id.eq(user))
                .first::<Item>(self)?,
            None => settings::dsl::settings
                .filter(settings::dsl::key.eq(&key))
                .filter(settings::dsl::user_id.is_null())
                .first::<Item>(self)?,
        };

        Ok((it.value, it.nonce))
    }

    fn set<K: Display>(
        &mut self,
        key: &K,
        user: Option<i32>,
        value: &[u8],
        nonce: Option<&[u8]>,
    ) -> Result<()> {
        let key = key.to_string();
        let now = Utc::now().naive_utc();

        let it = match user {
            Some(user) => settings::dsl::settings
                .filter(settings::dsl::key.eq(&key))
                .filter(settings::dsl::user_id.eq(user))
                .first::<Item>(self),
            None => settings::dsl::settings
                .filter(settings::dsl::key.eq(&key))
                .filter(settings::dsl::user_id.is_null())
                .first::<Item>(self),
        };

        match it {
            Ok(it) => {
                let it = settings::dsl::settings.filter(settings::dsl::id.eq(&it.id));

                update(it)
                    .set((
                        settings::dsl::value.eq(value),
                        settings::dsl::nonce.eq(nonce),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(settings::dsl::settings)
                    .values((
                        settings::dsl::key.eq(&key),
                        settings::dsl::user_id.eq(user),
                        settings::dsl::value.eq(value),
                        settings::dsl::nonce.eq(nonce),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
        };
        Ok(())
    }
}
