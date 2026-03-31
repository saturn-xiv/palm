use std::{ops::Deref, process, sync::Arc};

use diesel_adapter::{DieselAdapter, casbin::prelude::*};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::{
    runtime::Handle,
    sync::Mutex,
    task::spawn,
    time::{Duration, sleep},
};

pub use casbin::{CoreApi, Enforcer, RbacApi};

use super::{
    HttpError, Result,
    orm::postgresql::Pool as PostgreSqlPool,
    queue::{
        FlexbuffersConsumer,
        rabbitmq::{
            BasicPublishOptions, Client as RabbitMqClient, ExchangeDeclareOptions, ExchangeKind,
            FlexBuffersMessage, FlexbuffersConsumer as RabbitMqFlexbuffersConsumer,
            QueueBindOptions, QueueDeclareOptions,
        },
    },
    random::uuid,
};

// https://casbin.apache.org/
pub async fn postgresql_rabbitmq_enforcer(
    db: PostgreSqlPool,
    queue: Arc<RabbitMqClient>,
) -> Result<Arc<Mutex<Enforcer>>> {
    let id = uuid();
    log::info!("open casbin enforcer {}", id);
    let enforcer = Arc::new(Mutex::new({
        let model = DefaultModel::from_str(RBAC_MODEL).await?;
        let adapter = DieselAdapter::with_pool(db)?;
        let watcher = RabbitMqWatcher::new(&id, queue.clone(), QUEUE).await?;
        let mut it = Enforcer::new(model, adapter).await?;
        it.set_watcher(Box::new(watcher));
        it
    }));
    {
        let pid = process::id();
        let client = queue.clone();
        let consumer = RabbitMqFlexbuffersConsumer {
            handler: RabbitMqWatcherConsumer {
                id: id.clone(),
                enforcer: enforcer.clone(),
            },
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
    client: Arc<RabbitMqClient>,
    exchange: String,
    id: String,
}

impl RabbitMqWatcher {
    pub async fn new(id: &str, client: Arc<RabbitMqClient>, exchange: &str) -> Result<Self> {
        log::debug!("open casbin watcher for {}", id);
        client
            .declare_exchange(
                exchange,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
            )
            .await
            .map_err(|e| HttpError(StatusCode::INTERNAL_SERVER_ERROR, Some(e.to_string())))?;
        Ok(Self {
            id: id.to_string(),
            client,
            exchange: exchange.to_string(),
        })
    }
}

impl Watcher for RabbitMqWatcher {
    fn set_update_callback(&mut self, _cb: Box<dyn FnMut(String) + Send + Sync>) {
        log::debug!("set casbin-watcher({}) callback", self.id);
    }
    fn update(&mut self, event: casbin::EventData) {
        log::debug!("casbin event({}): {}", self.id, event);

        let handle = Handle::current();
        let id = self.id.clone();
        let client = self.client.clone();
        let exchange = self.exchange.clone();
        handle.spawn(async move {
            if let Err(e) = FlexBuffersMessage::publish(
                client.deref(),
                &exchange,
                "",
                &WatcherMessage { id },
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
    pub id: String,
    pub enforcer: Arc<Mutex<Enforcer>>,
}

impl FlexbuffersConsumer for RabbitMqWatcherConsumer {
    type Message = WatcherMessage;
    async fn consume(&self, _id: &str, task: &Self::Message) -> Result<()> {
        if task.id != self.id {
            let mut it = self.enforcer.lock().await;
            it.load_policy().await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherMessage {
    pub id: String,
}
