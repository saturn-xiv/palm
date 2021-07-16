use std::collections::HashSet;
use std::ops::Add;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use hyper::http::StatusCode;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::super::super::super::{orm::postgresql::Connection, HttpError, Result};
use super::{
    operation::Dao as OperationDao, resource::Dao as ResouceDao, role::Dao as RoleDao,
    schema::policies,
};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub role_id: i32,
    pub resource_id: i32,
    pub operation_id: i32,
    pub not_before: NaiveDate,
    pub expire_at: NaiveDate,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, GraphQLObject, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub role: String,
    pub resource: String,
    pub operation: String,
}

impl Item {
    pub fn enable(&self) -> bool {
        let today = Utc::now().naive_utc().date();
        today.ge(&self.not_before) && today.le(&self.expire_at)
    }
    pub fn weeks(d: i64) -> (NaiveDate, NaiveDate) {
        let nbf = Utc::now().naive_utc();
        let exp = nbf.add(Duration::weeks(d));
        (nbf.date(), exp.date())
    }

    pub fn years(y: i32) -> Result<(NaiveDate, NaiveDate)> {
        let nbf = Utc::now().naive_utc();
        if let Some(exp) = nbf.with_year(nbf.year() + y) {
            return Ok((nbf.date(), exp.date()));
        }
        Err(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad year gap!".to_string()),
        )))
    }
}

pub trait Dao {
    fn by_role(&self, id: i32) -> Result<Vec<Item>>;
    fn by_operation(&self, id: i32) -> Result<Vec<Item>>;
    fn by_resource(&self, id: i32) -> Result<Vec<Item>>;
    fn by_user(&self, id: i32) -> Result<Vec<Item>>;
    fn entities(&self, user: i32) -> Result<Vec<Entity>>;
    fn get(&self, role: i32, resource: i32, operation: i32) -> Result<Item>;
    fn deny(&self, role: i32, resource: i32, operation: i32) -> Result<()>;
    fn apply(
        &self,
        role: i32,
        resource: i32,
        operation: i32,
        not_before: &NaiveDate,
        expire_at: &NaiveDate,
    ) -> Result<()>;
    fn can(&self, user: i32, resource: &str, operation: &str) -> bool;
}

impl Dao for Connection {
    fn by_user(&self, id: i32) -> Result<Vec<Item>> {
        let mut items = Vec::new();
        let mut roles: HashSet<i32> = HashSet::new();
        {
            for it in RoleDao::by_user(self, id)? {
                roles.insert(it.id);
                roles.extend(
                    RoleDao::offsprings(self, it.id)?
                        .iter()
                        .map(|x| x.id)
                        .collect::<Vec<i32>>(),
                );
            }
        }
        for role in roles {
            items.extend(Dao::by_role(self, role)?);
        }
        Ok(items)
    }
    fn by_role(&self, id: i32) -> Result<Vec<Item>> {
        let items = policies::dsl::policies
            .filter(policies::dsl::role_id.eq(id))
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_operation(&self, id: i32) -> Result<Vec<Item>> {
        let items = policies::dsl::policies
            .filter(policies::dsl::operation_id.eq(id))
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_resource(&self, id: i32) -> Result<Vec<Item>> {
        let items = policies::dsl::policies
            .filter(policies::dsl::resource_id.eq(id))
            .load::<Item>(self)?;
        Ok(items)
    }
    fn entities(&self, user: i32) -> Result<Vec<Entity>> {
        let mut items = Vec::new();
        for it in Dao::by_user(self, user)? {
            if it.enable() {
                let resource = ResouceDao::by_id(self, it.resource_id)?;
                let role = RoleDao::by_id(self, it.role_id)?;
                let operation = OperationDao::by_id(self, it.operation_id)?;
                items.push(Entity {
                    role: role.code,
                    resource: resource.code,
                    operation: operation.code,
                });
            }
        }
        Ok(items)
    }
    fn get(&self, role: i32, resource: i32, operation: i32) -> Result<Item> {
        let it = policies::dsl::policies
            .filter(policies::dsl::role_id.eq(role))
            .filter(policies::dsl::resource_id.eq(resource))
            .filter(policies::dsl::operation_id.eq(operation))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn deny(&self, role: i32, resource: i32, operation: i32) -> Result<()> {
        delete(
            policies::dsl::policies
                .filter(policies::dsl::role_id.eq(role))
                .filter(policies::dsl::resource_id.eq(resource))
                .filter(policies::dsl::operation_id.eq(operation)),
        )
        .execute(self)?;
        Ok(())
    }
    fn apply(
        &self,
        role: i32,
        resource: i32,
        operation: i32,
        not_before: &NaiveDate,
        expire_at: &NaiveDate,
    ) -> Result<()> {
        let now = Utc::now().naive_local();
        match policies::dsl::policies
            .filter(policies::dsl::role_id.eq(role))
            .filter(policies::dsl::resource_id.eq(resource))
            .filter(policies::dsl::operation_id.eq(operation))
            .first::<Item>(self)
        {
            Ok(it) => {
                let it = policies::dsl::policies.filter(policies::dsl::id.eq(&it.id));
                update(it)
                    .set((
                        policies::dsl::not_before.eq(not_before),
                        policies::dsl::expire_at.eq(expire_at),
                        policies::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(policies::dsl::policies)
                    .values((
                        policies::dsl::role_id.eq(role),
                        policies::dsl::resource_id.eq(resource),
                        policies::dsl::operation_id.eq(operation),
                        policies::dsl::not_before.eq(not_before),
                        policies::dsl::expire_at.eq(expire_at),
                        policies::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
        }
        Ok(())
    }
    fn can(&self, user: i32, resource: &str, operation: &str) -> bool {
        fn f(db: &Connection, user: i32, resource: &str, operation: &str) -> Result<bool> {
            let resource = ResouceDao::by_code(db, resource)?;
            let operation = OperationDao::by_code(db, operation)?;
            for it in Dao::by_user(db, user)? {
                if it.resource_id == resource.id && it.operation_id == operation.id && it.enable() {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        f(self, user, resource, operation).unwrap_or_default()
    }
}
