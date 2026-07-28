use std::ops::DerefMut;
use std::path::Path;

use diesel::prelude::*;
use hyacinth::schema::email_users;
use portal::{
    Result, graphql::user::Item as User, models::user::email::Item as EmailUserItem,
    orm::postgresql::Node as PostgreSql, parse_toml,
};
use serde::{Deserialize, Serialize};

pub fn execute<P: AsRef<Path>>(config: P) -> Result<()> {
    let config: Config = parse_toml(config)?;
    let mut items = Vec::new();
    let db = config.postgresql.open()?;
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        for it in email_users::dsl::email_users
            .order(email_users::dsl::updated_at.desc())
            .load::<EmailUserItem>(db)?
            .iter()
        {
            items.push(User::by_email(db, it)?);
        }
    }

    println!("{:<36} {:<17} LABEL", "UID", "TYPE");
    for it in items {
        println!("{:<36} {:<17} {}", it.uid, it.r#type, it.label);
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    postgresql: PostgreSql,
}
