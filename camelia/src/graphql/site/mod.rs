pub mod info;
pub mod seo;
pub mod status;

use std::any::type_name;

use casbin::Enforcer;
use palm::{
    cache::redis::ClusterConnection as Cache, crypto::Secret, jwt::Jwt, session::Session, Error,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::setting::{Dao as SettingDao, FlatBuffer},
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};

pub async fn get<J: Jwt, S: Secret, V: DeserializeOwned + Default>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    aes: &S,
) -> Result<V, Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;
    let it = FlatBuffer::get(db, aes, None)?;
    Ok(it)
}

pub async fn set<J: Jwt, S: Secret, V: Serialize + Validate>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    aes: &S,
    item: &V,
) -> Result<(), Error> {
    item.validate()?;
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;
    FlatBuffer::set(db, aes, None, item, false)?;
    Ok(())
}

pub async fn delete<J: Jwt, S: Secret, V: Serialize>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    aes: &S,
) -> Result<(), Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;

    {
        let (id, _) = SettingDao::get(db, aes, &type_name::<V>().to_string(), None)?;
        SettingDao::delete(db, id)?;
    }
    Ok(())
}
