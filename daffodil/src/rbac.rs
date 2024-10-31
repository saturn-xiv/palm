use std::any::type_name;
use std::ops::Deref;
use std::sync::Arc;

use casbin::{prelude::*, EventData, Watcher};
use diesel_adapter::DieselAdapter;
use petunia::{
    orm::postgresql::Pool,
    queue::amqp::{ExchangeKind, Protobuf as ProtobufMessage, RabbitMq},
    rbac::v1::WatcherMessage,
    Result,
};
use tokio::sync::Mutex;

pub async fn new(db: Pool, queue: Arc<RabbitMq>) -> Result<Arc<Mutex<Enforcer>>> {
    let model = DefaultModel::from_str(include_str!("rbac_with_resource_roles_model.conf")).await?;
    let adapter = DieselAdapter::new(db)?;

    {
        let queue = queue.deref();
        let ch = queue.open().await?;
        RabbitMq::exchange_declare(
            &ch,
            type_name::<WatcherMessage>(),
            ExchangeKind::Fanout,
            false,
        )
        .await?;
    }

    let watcher_id = WatcherMessage::this_id()?;
    log::info!("start casbin watcher({watcher_id})");
    let enforcer = {
        let mut it = Enforcer::new(model, adapter).await?;
        it.set_watcher(Box::new(RabbitMqWatcher {
            queue,
            callback: Box::new(|| {}),
            id: watcher_id,
        }));
        it
    };
    let enforcer = Arc::new(Mutex::new(enforcer));
    Ok(enforcer)
}

pub struct RabbitMqWatcher {
    queue: Arc<RabbitMq>,
    callback: Box<dyn FnMut() + Send + Sync>,
    id: String,
}

impl Watcher for RabbitMqWatcher {
    fn set_update_callback(&mut self, cb: Box<dyn FnMut() + Send + Sync>) {
        log::warn!("reset casbin update callback");
        self.callback = cb;
    }
    fn update(&mut self, ev: EventData) {
        log::debug!("casbin event: {ev}");
        let queue = self.queue.deref();
        let by = self.id.clone();
        futures::executor::block_on(async move {
            if let Err(e) = ProtobufMessage::publish(queue, &WatcherMessage { by }).await {
                log::error!("publish casbin watcher message: {:?}", e);
            }
        });
    }
}
