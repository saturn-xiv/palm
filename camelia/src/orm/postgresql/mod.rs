pub mod schema;

use std::default::Default;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use casbin::prelude::*;
use diesel::{sql_query, RunQueryDsl};
use diesel_adapter::DieselAdapter;
use log::{debug, error};
use palm::{
    queue::rabbitmq::amqp::{
        watcher::{consume as consume_casbin_watcher, Watcher as RabbitCasbinWatcher},
        RabbitMq,
    },
    Result,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

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
    pub pool_size: u32,
}

impl Config {
    pub fn open(&self) -> Result<Pool> {
        let manager = diesel::r2d2::ConnectionManager::<Connection>::new(&self.to_string()[..]);
        Ok(Pool::builder().max_size(self.pool_size).build(manager)?)
    }
    pub async fn casbin_enforcer(&self, rabbitmq: Arc<RabbitMq>) -> Result<Arc<Mutex<Enforcer>>> {
        debug!("init casbin postgresql enforcer");
        let enforcer = {
            let model =
                DefaultModel::from_str(include_str!("rbac_with_resource_roles_model.conf")).await?;
            let url = self.to_string();
            debug!("init casbin rabbitmq enforcer");
            let adapter = DieselAdapter::new(&url, 4)?;
            let it = Enforcer::new(model, adapter).await?;
            Arc::new(Mutex::new(it))
        };

        debug!("init casbin rabbitmq adapter");
        let watcher = RabbitCasbinWatcher::new(rabbitmq.clone()).await?;
        {
            let rabbitmq = rabbitmq.clone();
            let queue = watcher.queue.clone();
            let enforcer = enforcer.clone();
            tokio::task::spawn(async move {
                let rabbitmq = rabbitmq.deref();
                if let Err(e) = consume_casbin_watcher("", rabbitmq, &queue, enforcer).await {
                    error!("consume casbin watcher message {:?}", e);
                }
            });
        }
        {
            let enforcer = enforcer.clone();
            let mut enf = enforcer.lock().await;
            enf.set_watcher(Box::new(watcher));
        }
        Ok(enforcer)
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
            pool_size: 32,
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
