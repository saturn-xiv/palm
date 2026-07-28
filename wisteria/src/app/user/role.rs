use std::ops::DerefMut;
use std::path::Path;

use hyacinth::rbac_v1::{
    UserRoleRequest,
    subject::{
        Role, User,
        role::{Administrator, Root},
    },
};
use portal::{
    Dahlia, HttpError, Result, models::user::Dao as UserDao, orm::postgresql::Node as PostgreSql,
    parse_toml,
};
use serde::{Deserialize, Serialize};

use super::super::http::Rpc;

pub async fn add<P: AsRef<Path>>(config: P, user: &str, role: &str) -> Result<()> {
    let config: Config = parse_toml(config)?;
    let db = config.postgresql.open()?;
    let mut db = db.get()?;
    let db = db.deref_mut();

    let mut req = UserRoleRequest::default();
    req.set_role(role_from_str(role));
    req.set_user({
        let it = UserDao::by_uid(db, user)?;
        let mut user = User::default();
        user.set_id(it.id);
        user
    });
    let dahlia = Dahlia::new(config.dahlia.open());
    dahlia
        .enforcer
        .add_role_for_user(req)
        .await
        .map_err(|x| Box::<HttpError>::new(x.into()))?;
    log::info!("done.");
    Ok(())
}

pub async fn delete<P: AsRef<Path>>(config: P, user: &str, role: &str) -> Result<()> {
    let config: Config = parse_toml(config)?;
    let db = config.postgresql.open()?;
    let mut db = db.get()?;
    let db = db.deref_mut();

    let mut req = UserRoleRequest::default();
    req.set_role(role_from_str(role));
    req.set_user({
        let it = UserDao::by_uid(db, user)?;
        let mut user = User::default();
        user.set_id(it.id);
        user
    });
    let dahlia = Dahlia::new(config.dahlia.open());
    dahlia
        .enforcer
        .delete_role_for_user(req)
        .await
        .map_err(|x| Box::<HttpError>::new(x.into()))?;
    log::info!("done.");
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    postgresql: PostgreSql,
    dahlia: Rpc,
}

fn role_from_str(s: &str) -> Role {
    let mut it = Role::default();
    match s {
        "administrator" => it.set_administrator(Administrator::default()),
        "root" => it.set_root(Root::default()),
        code => it.set_code(code),
    }
    it
}
