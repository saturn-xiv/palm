pub mod context;
pub mod controllers;
pub mod mutation;
pub mod query;

use juniper::{EmptySubscription, RootNode};

pub type Schema =
    RootNode<'static, query::Query, mutation::Mutation, EmptySubscription<context::Context>>;
