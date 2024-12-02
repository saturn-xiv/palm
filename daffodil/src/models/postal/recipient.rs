use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use juniper::GraphQLObject;
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::super::schema::postal_recipients;

#[derive(GraphQLObject, Queryable, Serialize, Deserialize, Clone)]
#[graphql(name = "PostalRecipient")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub whatsapp: Option<String>,
    pub wechat: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn create(
        &mut self,
        name: &str,
        tel: (Option<&str>, Option<&str>),
        internet: (Option<&str>, Option<&str>, Option<&str>),
    ) -> Result<i32>;
    fn update(
        &mut self,
        id: i32,
        name: &str,
        tel: (Option<&str>, Option<&str>),
        internet: (Option<&str>, Option<&str>, Option<&str>),
    ) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = postal_recipients::dsl::postal_recipients
            .filter(postal_recipients::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        name: &str,
        (phone, fax): (Option<&str>, Option<&str>),
        (email, whatsapp, wechat): (Option<&str>, Option<&str>, Option<&str>),
    ) -> Result<i32> {
        let now = Utc::now().naive_utc();
        let id = insert_into(postal_recipients::dsl::postal_recipients)
            .values((
                postal_recipients::dsl::name.eq(name),
                postal_recipients::dsl::fax.eq(fax),
                postal_recipients::dsl::phone.eq(phone),
                postal_recipients::dsl::email.eq(email),
                postal_recipients::dsl::wechat.eq(wechat),
                postal_recipients::dsl::whatsapp.eq(whatsapp),
                postal_recipients::dsl::updated_at.eq(&now),
            ))
            .returning(postal_recipients::dsl::id)
            .get_result(self)?;
        Ok(id)
    }
    fn update(
        &mut self,
        id: i32,
        name: &str,
        (phone, fax): (Option<&str>, Option<&str>),
        (email, whatsapp, wechat): (Option<&str>, Option<&str>, Option<&str>),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            postal_recipients::dsl::postal_recipients.filter(postal_recipients::dsl::id.eq(id));
        update(it)
            .set((
                postal_recipients::dsl::name.eq(name),
                postal_recipients::dsl::fax.eq(fax),
                postal_recipients::dsl::phone.eq(phone),
                postal_recipients::dsl::email.eq(email),
                postal_recipients::dsl::wechat.eq(wechat),
                postal_recipients::dsl::whatsapp.eq(whatsapp),
                postal_recipients::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            postal_recipients::dsl::postal_recipients.filter(postal_recipients::dsl::id.eq(id));
        update(it)
            .set((
                postal_recipients::dsl::deleted_at.eq(&Some(now)),
                postal_recipients::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            postal_recipients::dsl::postal_recipients.filter(postal_recipients::dsl::id.eq(id));
        update(it)
            .set((
                postal_recipients::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                postal_recipients::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
