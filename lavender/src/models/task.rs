use std::process::Output as ProcessOutput;
use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use hyacinth::schema::lavender_tasks;
use portal::{Result, orm::postgresql::Connection};
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_value};
use uuid::Uuid;

use super::job::Item as Job;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub elapsed: u128,
}

impl Output {
    pub fn new(it: &ProcessOutput, elapsed: Duration) -> Result<Self> {
        let stdout = std::str::from_utf8(&it.stdout)?;
        let stderr = std::str::from_utf8(&it.stderr)?;
        let it = Self {
            code: it.status.code(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            elapsed: elapsed.as_micros(),
        };

        Ok(it)
    }
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub uid: String,
    pub email: String,
    pub job: Value,
    pub script: String,
    pub args: Value,
    pub output: Option<Value>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn count(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn create<A: Into<String>>(
        &mut self,
        email: &str,
        job: &Job,
        script: &str,
        args: Vec<A>,
    ) -> Result<String>;
    fn set_output(&mut self, id: i64, output: &Output) -> Result<()>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
}

impl Dao for Connection {
    fn count(&mut self) -> Result<i64> {
        let it: i64 = lavender_tasks::dsl::lavender_tasks
            .count()
            .get_result(self)?;
        Ok(it)
    }
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = lavender_tasks::dsl::lavender_tasks
            .order(lavender_tasks::dsl::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn create<A: Into<String>>(
        &mut self,
        email: &str,
        job: &Job,
        script: &str,
        args: Vec<A>,
    ) -> Result<String> {
        let uid = Uuid::new_v4().to_string();
        let job = to_value(job)?;
        let args = {
            let it: Vec<String> = args.into_iter().map(|x| x.into()).collect();
            to_value(&it)?
        };
        let now = Utc::now().naive_utc();
        insert_into(lavender_tasks::dsl::lavender_tasks)
            .values((
                lavender_tasks::dsl::uid.eq(&uid),
                lavender_tasks::dsl::email.eq(email),
                lavender_tasks::dsl::job.eq(&job),
                lavender_tasks::dsl::script.eq(script),
                lavender_tasks::dsl::args.eq(&args),
                lavender_tasks::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(uid)
    }
    fn set_output(&mut self, id: i64, output: &Output) -> Result<()> {
        let now = Utc::now().naive_utc();
        let output = to_value(output)?;
        let it = lavender_tasks::dsl::lavender_tasks.filter(lavender_tasks::dsl::id.eq(id));
        update(it)
            .set((
                lavender_tasks::dsl::output.eq(&output),
                lavender_tasks::dsl::version.eq(lavender_tasks::dsl::version + 1),
                lavender_tasks::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = lavender_tasks::dsl::lavender_tasks
            .filter(lavender_tasks::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = lavender_tasks::dsl::lavender_tasks
            .filter(lavender_tasks::dsl::uid.eq(uid))
            .first::<Item>(self)?;
        Ok(it)
    }
}
