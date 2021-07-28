pub mod log;
pub mod schema;
pub mod site;

use super::super::super::orm::migration::Migration;

pub const UP: &str = include_str!("up.sql");
pub const DOWN: &str = include_str!("down.sql");

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-crawlers",
        version: "20210728125437",
        up: UP,
        down: DOWN
    };
}
