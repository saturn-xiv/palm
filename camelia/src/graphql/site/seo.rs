use casbin::Enforcer;
use juniper::GraphQLObject;
use palm::{
    cache::redis::ClusterConnection as Cache, crypto::Secret, jwt::Jwt, session::Session, Error,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::super::{
    models::setting::FlatBuffer, orm::postgresql::Connection as Db, services::CurrentUserAdapter,
};

#[derive(GraphQLObject, Validate, Serialize, Deserialize, Default)]
#[graphql(name = "BaiduSiteVerification")]
pub struct BaiduSiteVerification {
    #[validate(length(min = 1, max = 127))]
    pub content_code: String,
}

impl BaiduSiteVerification {
    pub async fn get<J: Jwt, S: Secret>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        aes: &S,
    ) -> Result<Self, Error> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(enf).await?;
        let it = FlatBuffer::get(db, aes, None)?;
        Ok(it)
    }

    pub async fn set<J: Jwt, S: Secret>(
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
        FlatBuffer::set(db, aes, None, self, false)?;
        Ok(())
    }
}
