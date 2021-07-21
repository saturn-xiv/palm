use std::process::Command;

use validator::Validate;

use super::super::super::super::{
    crypto::Aes, jwt::Jwt, orm::sqlite::Connection as Db, request::Token, Result,
};
use super::user::CurrentUser;

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(length(min = 1))]
    pub server: Option<String>,
}

impl Form {
    pub fn execute(&self, ss: &Token, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<String> {
        self.validate()?;
        ss.current_user(db, jwt, aes)?;
        let out = match self.server {
            Some(ref it) => Command::new("dig")
                .arg(&format!("@{}", it))
                .arg(&self.host)
                .output(),
            None => Command::new("dig").arg(&self.host).output(),
        }?;
        debug!("{:?}", out);
        Ok(String::from_utf8(out.stdout)?)
    }
}
