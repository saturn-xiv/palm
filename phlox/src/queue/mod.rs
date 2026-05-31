pub mod rabbitmq;

use std::fmt::Debug;
use std::future::Future;
use std::result::Result as StdResult;

use protobuf::Message as ProtobufMessage;
use serde::de::DeserializeOwned;

use super::Result;

pub trait Consumer {
    type Error: Debug;
    fn consume(
        &self,
        id: &str,
        content_type: &str,
        payload: &[u8],
    ) -> impl Future<Output = StdResult<(), Self::Error>> + Send;
}

pub trait ProtobufConsumer: Sync {
    type Message: ProtobufMessage + Default;
    fn consume(&self, id: &str, task: Self::Message) -> impl Future<Output = Result<()>> + Send;
}

pub trait FlexbuffersConsumer: Sync {
    type Message: DeserializeOwned + Send;
    fn consume(&self, id: &str, task: Self::Message) -> impl Future<Output = Result<()>> + Send;
}
