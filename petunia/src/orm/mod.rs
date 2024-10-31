pub mod mysql;
pub mod postgresql;
pub mod sqlite3;

use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    sql_types::{Text as DieselText, Timestamp as DieselTimestamp},
};

use super::Result;

#[derive(QueryableByName)]
pub struct Version {
    #[diesel(sql_type = DieselText)]
    pub value: String,
}

#[derive(QueryableByName)]
pub struct Timestamp {
    #[diesel(sql_type = DieselTimestamp)]
    pub value: NaiveDateTime,
}

pub trait Dao {
    fn version(&mut self) -> Result<String>;
    fn timestamp(&mut self) -> Result<NaiveDateTime>;
}
