pub mod baidu;

use casbin::Enforcer;
use palm::{
    cache::redis::ClusterConnection as Cache, jwt::Jwt, seo::baidu::ping::MethodCall as BaiduPing,
    session::Session, Error,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::super::{
    i18n::I18n, models::locale::Dao as LocaleDao, orm::postgresql::Connection as Db,
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
            let it = BaiduPing::new(&self.home, &title, lang);
            it.ping().await?;
        }
        Ok(())
    }
}
