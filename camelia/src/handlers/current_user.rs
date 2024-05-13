use std::ops::{Deref, DerefMut};
use std::result::Result as StdResult;

use actix_web::{dev::Payload, error::ErrorForbidden, web, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use hibiscus::handlers::token::Token;
use palm::{camelia::v1::UserTokenAction, openssl::jwt::Jwt, try_web, Jwt as JwtProvider, Result};

use super::super::{
    models::user::{session::Dao as UserSessionDao, Dao as UserDao, Item as User},
    orm::postgresql::{Connection as Db, Pool as DbPool},
    NAME,
};

fn user_from_token<P: JwtProvider>(token: &str, db: &mut Db, jwt: &P) -> Result<User> {
    let (_, uid, _) = jwt.verify::<String>(token, NAME, UserTokenAction::SignIn.as_str_name())?;
    let ss = UserSessionDao::by_uid(db, &uid)?;
    let user = UserDao::by_id(db, ss.user_id)?;
    user.available()?;
    Ok(user)
}

impl FromRequest for User {
    type Error = Error;
    type Future = Ready<StdResult<Self, Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let token = Token::detect(req);
        if let Some(jwt) = req.app_data::<web::Data<Jwt>>() {
            if let Some(db) = req.app_data::<web::Data<DbPool>>() {
                if let Ok(mut db) = try_web!(db.get()) {
                    let db = db.deref_mut();

                    if let Some(ref token) = token {
                        let jwt = jwt.deref();
                        let jwt = jwt.deref();
                        if let Ok(it) = user_from_token(token, db, jwt) {
                            return ok(it);
                        }
                    }
                }
            }
        }

        err(ErrorForbidden("can't fetch current user"))
    }
}
