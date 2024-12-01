pub mod controllers;
pub mod graphql;
pub mod models;
pub mod rbac;
pub mod schema;
pub mod session;

pub const NAME: &str = env!("CARGO_PKG_NAME");
