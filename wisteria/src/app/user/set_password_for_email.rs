use std::{ops::DerefMut, path::Path};

use diesel::Connection as DieselConnection;
use portal::{
    Error, Loquat, PasswordHashing, Result, current_user,
    graphql::{Plugin, user::email::SetPassword},
    hostname,
    models::{
        log::{Dao as LogDao, Level},
        user::email::Dao as EmailUserDao,
    },
    orm::postgresql::Node as PostgreSql,
    parse_toml,
};
use serde::{Deserialize, Serialize};

use super::super::http::Rpc;

pub async fn execute<P: AsRef<Path>>(config: P, email: &str, password: &str) -> Result<()> {
    let manager = current_user()?;
    let ip = hostname()?;

    let form = SetPassword {
        email: email.trim().to_lowercase(),
        password: password.to_string(),
    };
    let config: Config = parse_toml(config)?;
    let db = config.postgresql.open()?;
    let mut db = db.get()?;
    let db = db.deref_mut();
    let hashing = Loquat::new(config.loquat.open());
    let password = hashing.sign(&form.password).await?;

    db.transaction::<_, Error, _>(|tx| {
        let it = EmailUserDao::by_email(tx, &form.email)?;
        log::warn!("set user {}<{}> password", it.name, it.email);
        EmailUserDao::set_password(tx, it.id, &password)?;
        LogDao::create::<Plugin, _>(
            tx,
            it.user_id,
            Level::Info,
            &ip,
            format!("set password by system operator({}).", manager),
        )?;
        Ok(())
    })?;
    log::info!("done.");
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    postgresql: PostgreSql,
    loquat: Rpc,
}
