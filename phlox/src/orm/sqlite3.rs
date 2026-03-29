use diesel::{
    connection::Connection as DieselConnection, dsl::sql_query, prelude::*, result::QueryResult,
};
use serde::{Deserialize, Serialize};

use super::super::Result;
use super::Heartbeat;

pub type Connection = SqliteConnection;

// https://sqlite.org/lang_expr.html#varparam
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(default = "node_default_file")]
    pub file: String,
}

fn node_default_file() -> String {
    "db.sqlite3".to_string()
}

impl Node {
    // https://sqlite.org/wal.html
    pub fn open(&self) -> Result<Connection> {
        log::info!("open sqlite3 {}", self.file);
        let mut it = SqliteConnection::establish(&self.file)?;
        {
            sql_query("PRAGMA journal_mode=WAL").execute(&mut it)?;
            sql_query("PRAGMA busy_timeout = 5000").execute(&mut it)?;
        }
        Ok(it)
    }
}

impl super::Dao for Connection {
    fn heartbeat(&mut self) -> QueryResult<Heartbeat> {
        let it = sql_query("SELECT SQLITE_VERSION() AS version, CURRENT_TIMESTAMP AS now")
            .get_result(self)?;
        Ok(it)
    }
}
