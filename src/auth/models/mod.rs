pub mod schema;

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
