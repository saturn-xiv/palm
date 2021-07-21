use hyper::StatusCode;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::super::super::super::{
    crypto::Aes, jwt::Jwt, orm::sqlite::Connection, request::Token as Auth, HttpError, Result,
};
use super::super::super::nut::models::user::Token;
use super::super::models::settings::Dao as SettingDao;

#[derive(Serialize, Deserialize, Validate, GraphQLObject, PartialEq, Eq, Hash, Debug)]
#[graphql(name = "Administrator")]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[validate(length(min = 2))]
    pub name: String,
    #[validate(length(min = 6))]
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "admin".to_string(),
            password: "password".to_string(),
        }
    }
}

impl User {
    pub const KEY: &'static str = "administrator";

    pub fn sign_in(&self, db: &Connection, aes: &Aes) -> Result<()> {
        let it: Self = SettingDao::get(db, aes, Self::KEY)?;
        if it != *self {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        Ok(())
    }
    pub fn sign_out(&self) -> Result<()> {
        info!("sign out!");
        Ok(())
    }
    pub fn save(&self, db: &Connection, aes: &Aes) -> Result<()> {
        self.validate()?;
        SettingDao::set(db, aes, Self::KEY, &self.name, true)?;
        Ok(())
    }
}

pub trait CurrentUser {
    fn current_user(&self, db: &Connection, jwt: &Jwt, aes: &Aes) -> Result<User>;
}

impl CurrentUser for Auth {
    fn current_user(&self, db: &Connection, jwt: &Jwt, aes: &Aes) -> Result<User> {
        let token = jwt.parse::<Token>(&self.0)?;
        let token = token.claims;
        if token.act != Token::SIGN_IN {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        let it: User = SettingDao::get(db, aes, User::KEY)?;
        if it.name != token.uid {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        Ok(it)
    }
}
