use casbin::Enforcer;
use palm::{
    cache::redis::ClusterConnection as Cache, crypto::Secret, jwt::Jwt,
    seo::index_now::SiteVerification, session::Session, Error,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::super::super::{
    models::setting::FlatBuffer, orm::postgresql::Connection as Db, services::CurrentUserAdapter,
};

pub async fn get<J: Jwt, S: Secret>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    aes: &S,
) -> Result<SiteVerification, Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;
    let it = FlatBuffer::get(db, aes, None)?;
    Ok(it)
}

pub async fn set<J: Jwt, S: Secret>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    aes: &S,
    item: &SiteVerification,
) -> Result<(), Error> {
    item.validate()?;
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;
    FlatBuffer::set(db, aes, None, item, false)?;
    Ok(())
}
