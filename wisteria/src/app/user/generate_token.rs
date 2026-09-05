use std::{ops::DerefMut, path::Path};

use chrono::Duration;
use diesel::Connection as DieselConnection;
use portal::{
    Error, Jwt, Loquat, Result, current_user,
    graphql::{CurrentUser, Plugin},
    hostname,
    models::{
        log::{Dao as LogDao, Level},
        user::Dao as UserDao,
    },
    orm::postgresql::Node as PostgreSql,
    parse_toml,
};
use serde::{Deserialize, Serialize};

use super::super::http::Rpc;

pub async fn execute<P: AsRef<Path>>(
    config: P,
    uid: &str,
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

    let user = UserDao::by_uid(db, uid)?;
    log::warn!("generate a token({} weeks) for user {}", weeks, user.name);

    let token = Jwt::sign(
        &loquat,
        CurrentUser::ISSUER,
        uid,
        audiences.clone(),
        Duration::weeks(weeks as i64),
        None::<String>,
    )
    .await?;

    db.transaction::<_, Error, _>(|tx| {
        LogDao::create::<Plugin, _>(
            tx,
            user.id,
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
