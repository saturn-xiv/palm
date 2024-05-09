pub mod controllers;
// pub mod graphql;
pub mod handlers;
pub mod i18n;
pub mod models;
pub mod orm;
pub mod schema;
pub mod services;

pub const NAME: &str = env!("CARGO_PKG_NAME");
