use std::ops::Deref;
use std::sync::Arc;

use casbin::{prelude::*, EventData, Watcher};
use diesel_adapter::DieselAdapter;
use petunia::{
    orm::postgresql::Pool,
    queue::amqp::{Protobuf as ProtobufMessage, RabbitMq},
    rbac::v1::WatcherMessage,
    Result,
};
use tokio::{runtime::Builder as TokioRuntimeBuilder, sync::Mutex};

pub async fn new(db: Pool, queue: Arc<RabbitMq>) -> Result<Arc<Mutex<Enforcer>>> {
    let model =
        DefaultModel::from_file(include_str!("rbac_with_resource_roles_model.conf")).await?;
    let adapter = DieselAdapter::new(db)?;

    let enforcer = {
        let mut it = Enforcer::new(model, adapter).await?;
        it.set_watcher(Box::new(RabbitMqWatcher {
            queue,
            callback: Box::new(|| {}),
        }));
        it
    };
    let enforcer = Arc::new(Mutex::new(enforcer));
    Ok(enforcer)
}

pub struct RabbitMqWatcher {
    queue: Arc<RabbitMq>,
    callback: Box<dyn FnMut() + Send + Sync>,
}

impl Watcher for RabbitMqWatcher {
    fn set_update_callback(&mut self, cb: Box<dyn FnMut() + Send + Sync>) {
        log::warn!("reset reset casbin update callback");
        self.callback = cb;
    }
    fn update(&mut self, ev: EventData) {
        log::debug!("casbin event: {ev}");
        let queue = self.queue.deref();
        if let Ok(rt) = TokioRuntimeBuilder::new_current_thread()
            .enable_all()
            .build()
        {
            if let Err(e) = rt.block_on(ProtobufMessage::publish(queue, &WatcherMessage {})) {
                log::error!("publish casbin watcher message: {:?}", e);
            }
        }
    }
}
