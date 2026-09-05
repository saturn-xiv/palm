use std::{ops::DerefMut, path::Path};

use diesel::Connection as DieselConnection;
use portal::{
    Dahlia, Error, Loquat, PasswordHashing, Result, current_user,
    graphql::{Plugin, user::email::SignUp},
    hostname,
    models::{
        log::{Dao as LogDao, Level},
        user::{Dao as UserDao, email::Dao as EmailUserDao},
    },
    orm::postgresql::Node as PostgreSql,
    parse_toml,
};
use serde::{Deserialize, Serialize};

use super::super::http::Rpc;

pub async fn execute<P: AsRef<Path>>(config: P, uid: &str, weeks: u32) -> Result<()> {
    let manager = current_user()?;
    let ip = hostname()?;

    let config: Config = parse_toml(config)?;
    let db = config.postgresql.open()?;
    let mut db = db.get()?;
    let db = db.deref_mut();
    let rbac = Dahlia::new(config.dahlia.open());

    let user = UserDao::by_uid(db, uid)?;
    log::warn!(
        "generate a token({}, {} weeks) for user {}",
        form.name,
        form.email
    );

    db.transaction::<_, Error, _>(|tx| {
        form.create(tx, &password)?;
        let it = EmailUserDao::by_email(tx, &form.email)?;
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
    dahlia: Rpc,
}
