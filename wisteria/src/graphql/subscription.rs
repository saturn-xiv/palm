use std::{pin::Pin, time::Duration};

use futures::stream::{Stream, StreamExt as _};
use juniper::{FieldError, graphql_subscription};
use chrono::Utc;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

use super::context::Context;

type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

pub struct Subscription;

#[graphql_subscription]
#[graphql(context = Context)]
impl Subscription {
    pub async fn notifications(_ctx: &Context) -> StringStream {
        let stream = IntervalStream::new(interval(Duration::from_secs(3)))
            .map(move |_| Ok(format!("Hello, Marigold({})!", Utc::now())));
        Box::pin(stream)
    }
}
