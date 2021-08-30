pub mod attachment;
pub mod notification;
pub mod schema;
pub mod tag;

use super::super::super::orm::migration::Migration;

pub const UP: &str = include_str!("up.sql");
pub const DOWN: &str = include_str!("down.sql");

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-nut",
        version: "20210517125807",
        up: UP,
        down: DOWN
    };
}
