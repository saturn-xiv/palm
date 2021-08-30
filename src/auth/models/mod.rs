pub mod group;
pub mod log;
pub mod operation;
pub mod policy;
pub mod resource;
pub mod role;
pub mod schema;
pub mod user;

use super::super::orm::migration::Migration;

pub const UP: &str = include_str!("up.sql");
pub const DOWN: &str = include_str!("down.sql");

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-auth",
        version: "20210516234337",
        up: UP,
        down: DOWN
    };
}
