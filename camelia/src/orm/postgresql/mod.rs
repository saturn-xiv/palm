pub mod schema;

use std::default::Default;
use std::fmt;

use diesel::{sql_query, RunQueryDsl};
use hibiscus::Result;
use log::debug;
use serde::{Deserialize, Serialize};

use super::Version;

pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Connection>>;
pub type PooledConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<Connection>>;

// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
pub type Connection = diesel::pg::PgConnection;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: Option<String>,
    #[serde(rename = "pool-size")]
    pub pool_size: Option<u32>,
}

impl Config {
    pub fn open(&self) -> Result<Pool> {
        debug!(
            "open postgresql://{}@{}:{}/{}",
            self.user, self.host, self.port, self.name
        );
        let manager = diesel::r2d2::ConnectionManager::<Connection>::new(&self.to_string()[..]);
        Ok(Pool::builder()
            .max_size(self.pool_size.unwrap_or(32))
            .build(manager)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            name: "demo".to_string(),
            password: None,
            pool_size: Some(32),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.as_deref().unwrap_or(""),
            self.host,
            self.port,
            self.name
        )
    }
}

impl super::Dao for Connection {
    fn version(&mut self) -> Result<String> {
        let it: Version = sql_query("SELECT VERSION() AS value").get_result(self)?;
        Ok(it.value)
    }
}
