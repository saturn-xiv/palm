pub mod context;
pub mod controllers;
pub mod query;

use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub type Schema = RootNode<
    'static,
    query::Query,
    EmptyMutation<context::Context>,
    EmptySubscription<context::Context>,
>;
