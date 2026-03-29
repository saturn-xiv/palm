pub mod mysql;
pub mod postgresql;
pub mod sqlite3;

use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    sql_types::{Text, Timestamp},
};
use serde::{Deserialize, Serialize};

pub use diesel::QueryResult;

#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct Heartbeat {
    #[diesel(column_name=version, sql_type=Text)]
    version: String,
    #[diesel(column_name=now, sql_type=Timestamp)]
    created_at: NaiveDateTime,
}

pub trait Dao {
    fn heartbeat(&mut self) -> QueryResult<Heartbeat>;
}
