use std::result::Result as StdResult;

use diesel::{
    dsl::sql_query,
    prelude::*,
    r2d2::{ConnectionManager, Pool as DieselPool},
    result::QueryResult,
};
use r2d2::Error as R2d2Error;
use serde::{Deserialize, Serialize};

use super::Heartbeat;

pub type Connection = PgConnection;
pub type Pool = DieselPool<ConnectionManager<PgConnection>>;

// https://www.postgresql.org/docs/current/sql-prepare.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(default = "node_default_host")]
    pub host: String,
    #[serde(default = "node_default_port")]
    pub port: u16,
    #[serde(default = "node_default_user")]
    pub user: String,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(rename = "db-name")]
    pub db_name: String,
    #[serde(rename = "pool-size", default = "node_default_pool_size")]
    pub pool_size: usize,
}

fn node_default_host() -> String {
    "127.0.0.1".to_string()
}

fn node_default_user() -> String {
    "postgres".to_string()
}

fn node_default_port() -> u16 {
    5432
}

fn node_default_pool_size() -> usize {
    32
}

impl Node {
    pub fn open(&self) -> StdResult<Pool, R2d2Error> {
        log::debug!(
            "open PostgreSql {}@{}:{}/{}",
            self.user,
            self.host,
            self.port,
            self.db_name,
        );
        let url = self.url();
        let manager = ConnectionManager::<PgConnection>::new(&url);

        let it = DieselPool::builder()
            .max_size(self.pool_size as u32)
            .test_on_check_out(true)
            .build(manager)?;
        Ok(it)
    }
    // https://www.postgresql.org/docs/current/libpq-connect.html
    pub fn url(&self) -> String {
        let it = format!(
            "host={} port={} dbname={} user={} connect_timeout=5 sslmode=disable",
            self.host, self.port, self.db_name, self.user
        );
        match self.password {
            Some(ref s) => format!("{} password={}", it, s),
            None => it,
        }
    }
}

impl super::Dao for Connection {
    fn heartbeat(&mut self) -> QueryResult<Heartbeat> {
        let it =
            sql_query("SELECT VERSION() AS version, CURRENT_TIMESTAMP AS now").get_result(self)?;
        Ok(it)
    }
}
