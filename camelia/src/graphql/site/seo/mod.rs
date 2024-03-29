pub mod baidu;
pub mod google;
pub mod index_now;

use casbin::Enforcer;
use palm::{
    cache::redis::ClusterConnection as Cache,
    crypto::Secret,
    jwt::Jwt,
    seo::{
        baidu::ping::MethodCall as BaiduPingRequest,
        google::ping as ping_google,
        index_now::{
            ping::Request as IndexNowPingRequest, SiteVerification as IndexNowSiteVerification,
        },
    },
    session::Session,
    Error,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::super::{
    i18n::I18n,
    models::{locale::Dao as LocaleDao, setting::FlatBuffer},
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};
use super::info::Response;

#[derive(Validate)]
pub struct Ping {
    #[validate(url, length(min = 1, max = 127))]
    pub home: String,
}

impl Ping {
    pub async fn baidu<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(enf).await?;
        for lang in LocaleDao::languages(db)?.iter() {
            let title = I18n::t(db, lang, Response::TITLE, &None::<String>);
            let it = BaiduPingRequest::new(&self.home, &title, lang);
            it.ping().await?;
        }
        Ok(())
    }
    pub async fn google<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(enf).await?;
        ping_google(&self.home).await?;
        Ok(())
    }
    pub async fn index_now<J: Jwt, S: Secret>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        aes: &S,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(enf).await?;

        let profile: IndexNowSiteVerification = FlatBuffer::get(db, aes, None)?;
        let links = Vec::new(); // TODO
        for it in IndexNowPingRequest::new(&self.home, &profile.key, &links).iter() {
            it.ping().await?;
        }

        Ok(())
    }
}
