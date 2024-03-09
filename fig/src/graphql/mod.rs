pub mod context;
pub mod mutation;
pub mod query;
pub mod controllers;

use juniper::{EmptySubscription, RootNode};

pub type Schema = RootNode<'static, query::Query, mutation::Mutation, EmptySubscription>;
