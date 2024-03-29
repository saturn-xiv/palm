use casbin::Enforcer;
use palm::{
    cache::redis::ClusterConnection as Cache,
    crypto::Secret,
    jwt::Jwt,
    seo::baidu::{ping as ping_baidu, SiteVerification},
    session::Session,
    Error,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::super::super::{
    i18n::I18n,
    models::{locale::Dao as LocaleDao, setting::FlatBuffer},
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};
use super::super::info::Response;

pub async fn ping<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    home: &str,
) -> Result<(), Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;
    for lang in LocaleDao::languages(db)?.iter() {
        let title = I18n::t(db, lang, Response::TITLE, &None::<String>);
        ping_baidu(home, &title, lang).await?;
    }
    Ok(())
}

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
