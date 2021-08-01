pub mod log;
pub mod schema;

use super::super::super::orm::migration::Migration;

pub const UP: &str = include_str!("up.sql");
pub const DOWN: &str = include_str!("down.sql");

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-sms",
        version: "20210730180934",
        up: UP,
        down: DOWN
    };
}
