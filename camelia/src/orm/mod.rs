pub mod mysql;
pub mod postgresql;
pub mod sqlite;

use diesel::{sql_types::Text, QueryableByName};
use palm::Result;

#[derive(QueryableByName)]
pub struct Version {
    #[diesel(sql_type = Text)]
    pub value: String,
}

pub trait Dao {
    fn version(&mut self) -> Result<String>;
}
