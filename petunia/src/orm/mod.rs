pub mod mysql;
pub mod postgresql;
pub mod sqlite3;

use diesel::{prelude::*, sql_types::Text};

#[derive(QueryableByName)]
pub struct Version {
    #[diesel(sql_type = Text)]
    pub value: String,
}
