use std::any::type_name;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration as StdDuration;

use casbin::{CoreApi, Enforcer, EventData, Watcher as CasbinWatcher};
use rabbitmq_stream_client::types::OffsetSpecification;
use tokio::sync::Mutex;

use super::super::super::super::Result;
use super::super::CasbinSyncTask;
use super::{Client, Flatbuffer};

pub struct Watcher {
    rabbitmq: Arc<Client>,
    callback: Box<dyn FnMut() + Send + Sync>,
    pub queue: String,
}

impl Watcher {
    pub async fn new(rabbitmq: Arc<Client>) -> Result<Self> {
        let queue = type_name::<CasbinSyncTask>();
        rabbitmq.create_stream(queue, 1, 1).await?;
        info!("casbin watcher use stream {queue}");
        Ok(Self {
            rabbitmq,
            queue: queue.to_string(),
            callback: Box::new(|| {
                debug!("empty casbin callback");
            }),
        })
    }
    async fn handle_event(rabbitmq: &Client, event: EventData) -> Result<()> {
        debug!("casbin watcher receive event: {}", event);
        Flatbuffer::produce(rabbitmq, &CasbinSyncTask).await?;
        Ok(())
    }
}

impl CasbinWatcher for Watcher {
    fn set_update_callback(&mut self, callback: Box<dyn FnMut() + Send + Sync>) {
        debug!("update casbin watcher callback");
        self.callback = callback;
    }
    fn update(&mut self, event: EventData) {
        let rabbitmq = self.rabbitmq.clone();
        let rabbitmq = rabbitmq.deref();
        futures::executor::block_on(async move {
            if let Err(e) = Self::handle_event(rabbitmq, event).await {
                error!("publish casbin watcher message: {e:?}");
            }
        });
    }
}

pub async fn start_consumer(client: Arc<Client>, enforcer: Arc<Mutex<Enforcer>>) -> Result<()> {
    tokio::task::spawn(async move {
        loop {
            let enforcer = enforcer.clone();

            if let Err(e) = client
                .consume(
                    type_name::<CasbinSyncTask>(),
                    &Handler { enforcer },
                    OffsetSpecification::Next,
                )
                .await
            {
                error!("consume casbin watcher message {:?}", e);
                tokio::time::sleep(StdDuration::from_secs(1)).await
            }
        }
    });

    Ok(())
}

struct Handler {
    enforcer: Arc<Mutex<Enforcer>>,
}

impl super::Handler for Handler {
    async fn handle(&self, id: &str, _content_type: &str, _payload: &[u8]) -> Result<()> {
        debug!("reload casbin policies message {}", id);
        let mut enforcer = self.enforcer.lock().await;
        enforcer.load_policy().await?;
        Ok(())
    }
}
