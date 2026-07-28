use std::{ops::DerefMut, path::Path};

use diesel::Connection as DieselConnection;
use portal::{
    Error, Loquat, PasswordHashing, Result, current_user,
    graphql::{Plugin, user::email::SignUp},
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

pub async fn execute<P: AsRef<Path>>(
    config: P,
    name: &str,
    email: &str,
    password: &str,
) -> Result<()> {
    let manager = current_user()?;
    let ip = hostname()?;

    let form = SignUp {
        name: name.to_string(),
        email: email.to_string(),
        password: password.to_string(),
        lang: "en-US".to_string(),
        timezone: "UTC".to_string(),
    };
    let config: Config = parse_toml(config)?;
    let db = config.postgresql.open()?;
    let mut db = db.get()?;
    let db = db.deref_mut();
    let hashing = Loquat::new(config.loquat.open());
    let password = hashing.sign(&form.password).await?;

    log::warn!("create user {}<{}>", form.name, form.email);

    db.transaction::<_, Error, _>(|tx| {
        let email = form.create(tx, &password)?;
        let it = EmailUserDao::by_email(tx, &email)?;
        EmailUserDao::confirm(tx, it.id)?;
        LogDao::create::<Plugin, _>(
            tx,
            it.user_id,
            Level::Info,
            &ip,
            format!("created by system operator({}).", manager),
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
