pub mod attachment;
pub mod group;
pub mod log;
pub mod notification;
pub mod operation;
pub mod policy;
pub mod resource;
pub mod role;
pub mod schema;
pub mod tag;
pub mod user;

use super::super::super::orm::migration::Migration;

pub const UP: &str = include_str!("up.sql");
pub const DOWN: &str = include_str!("down.sql");

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-rbac",
        version: "20210517125807",
        up: UP,
        down: DOWN
    };
}
