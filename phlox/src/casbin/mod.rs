use std::{process, sync::Arc};

use diesel_adapter::{DieselAdapter, casbin::prelude::*};
use hyper::StatusCode;
use tokio::{
    runtime::Handle,
    sync::Mutex,
    task::spawn,
    time::{Duration, sleep},
};

use super::{
    Error, HttpError, Result,
    orm::postgresql::Pool as PostgreSqlPool,
    queue::{
        Consumer,
        rabbitmq::{
            BasicPublishOptions, Client as RabbitMqClient, ExchangeDeclareOptions, ExchangeKind,
            QueueBindOptions, QueueDeclareOptions,
        },
    },
};

// https://casbin.apache.org/
pub async fn postgresql_rabbitmq_enforcer(
    db: PostgreSqlPool,
    queue: Arc<RabbitMqClient>,
) -> Result<Arc<Mutex<Enforcer>>> {
    let enforcer = Arc::new(Mutex::new({
        let model = DefaultModel::from_str(RBAC_MODEL).await?;
        let adapter = DieselAdapter::with_pool(db)?;
        let watcher = RabbitMqWatcher::new(queue.clone(), QUEUE).await?;
        let mut it = Enforcer::new(model, adapter).await?;
        it.set_watcher(Box::new(watcher));
        it
    }));
    {
        let pid = process::id();
        let client = queue.clone();
        let consumer = RabbitMqWatcherConsumer {
            enforcer: enforcer.clone(),
        };
        let queue = client
            .declare_anonymous_queue(QueueDeclareOptions {
                auto_delete: true,
                exclusive: true,
                ..Default::default()
            })
            .await?;
        client
            .bind_queue(&queue, QUEUE, "*", QueueBindOptions::default())
            .await?;

        spawn(async move {
            loop {
                if let Err(e) = client
                    .consume(&format!("casbin.watcher.{}", pid), &queue, &consumer)
                    .await
                {
                    log::error!("{}", e);
                    sleep(Duration::from_mins(1)).await;
                }
            }
        });
    }
    Ok(enforcer)
}

pub const RBAC_MODEL: &str = include_str!("rbac_model.conf");
pub const QUEUE: &str = "casbin.watcher";

pub struct RabbitMqWatcher {
    pub client: Arc<RabbitMqClient>,
    pub exchange: String,
}

impl RabbitMqWatcher {
    pub async fn new(client: Arc<RabbitMqClient>, exchange: &str) -> Result<Self> {
        client
            .declare_exchange(
                exchange,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
            )
            .await
            .map_err(|e| HttpError(StatusCode::INTERNAL_SERVER_ERROR, Some(e.to_string())))?;
        Ok(Self {
            client,
            exchange: exchange.to_string(),
        })
    }
}

impl Watcher for RabbitMqWatcher {
    fn set_update_callback(&mut self, _cb: Box<dyn FnMut(String) + Send + Sync>) {
        log::debug!("set casbin-watcher callback");
    }
    fn update(&mut self, event: casbin::EventData) {
        let event = event.to_string();
        let content_type = mime::TEXT_PLAIN_UTF_8.to_string();
        let handle = Handle::current();
        handle.block_on(async {
            if let Err(e) = self
                .client
                .publish(
                    &self.exchange,
                    "",
                    &content_type,
                    event.as_bytes(),
                    BasicPublishOptions::default(),
                )
                .await
            {
                log::error!("{}", e);
            }
        });
    }
}

pub struct RabbitMqWatcherConsumer {
    pub enforcer: Arc<Mutex<Enforcer>>,
}

impl Consumer for RabbitMqWatcherConsumer {
    type Error = Error;
    async fn consume(&self, _id: &str, _content_type: &str, _payload: &[u8]) -> Result<()> {
        let mut it = self.enforcer.lock().await;
        it.load_policy().await?;
        Ok(())
    }
}
