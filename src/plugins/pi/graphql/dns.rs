use std::process::Command;

use validator::Validate;

use super::super::super::super::{graphql::Session, jwt::Jwt, orm::Connection as Db, Result};

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(length(min = 1))]
    pub server: Option<String>,
}

impl Form {
    pub fn execute(&self, ss: &Session, db: &Db, jwt: &Jwt) -> Result<String> {
        self.validate()?;
        ss.current_user(db, jwt)?;
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
