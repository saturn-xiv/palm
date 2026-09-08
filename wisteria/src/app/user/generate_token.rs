use std::{ops::DerefMut, path::Path};

use chrono::Duration;
use diesel::Connection as DieselConnection;
use portal::{
    Error, Loquat, Result, current_user,
    graphql::{CurrentUser, Plugin},
    hostname,
    models::{
        log::{Dao as LogDao, Level},
        user::{Type as UserType, email::Dao as EmailUserDao},
    },
    orm::postgresql::Node as PostgreSql,
    parse_toml,
};
use serde::{Deserialize, Serialize};

use super::super::http::Rpc;

pub async fn execute<P: AsRef<Path>>(
    config: P,
    email: &str,
    audiences: Vec<String>,
    weeks: u32,
) -> Result<()> {
    let manager = current_user()?;
    let ip = hostname()?;

    let config: Config = parse_toml(config)?;
    let db = config.postgresql.open()?;
    let mut db = db.get()?;
    let db = db.deref_mut();
    let loquat = Loquat::new(config.loquat.open());

    let email_user = EmailUserDao::by_email(db, email)?;
    log::warn!(
        "generate a token({} weeks) for user {}",
        weeks,
        email_user.name
    );

    let token = CurrentUser::token(
        &loquat,
        UserType::Email,
        &email_user.email,
        audiences.clone(),
        Duration::weeks(1),
    )
    .await?;

    db.transaction::<_, Error, _>(|tx| {
        LogDao::create::<Plugin, _>(
            tx,
            email_user.user_id,
            Level::Info,
            &ip,
            format!(
                "generate a token({} weeks, {}) by system operator({}).",
                weeks,
                audiences.join(","),
                manager
            ),
        )?;
        Ok(())
    })?;

    println!("TOKEN: {token}");
    log::info!("done.");
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    postgresql: PostgreSql,
    loquat: Rpc,
}
