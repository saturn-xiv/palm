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
}

impl Form {
    pub fn execute(&self, ss: &Token, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<String> {
        self.validate()?;
        ss.current_user(db, jwt, aes)?;
        let out = Command::new("ping")
            .arg("-W")
            .arg("2")
            .arg("-c")
            .arg("4")
            .arg(&self.host)
            .output()?;
        debug!("{:?}", out);
        Ok(String::from_utf8(out.stdout)?)
    }
}
