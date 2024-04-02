use clap::Parser;
use log::info;
use palm::{
    duration_from_days,
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    Result,
};

use super::super::{models::log::Item as Log, NAME};
use super::Config;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Generate {
    #[clap(short, long)]
    pub user: String,
    #[clap(short, long, default_value_t = 7)]
    pub days: i64,
}

impl Generate {
    pub fn generate(&self, config: &Config) -> Result<()> {
        let ttl = duration_from_days(self.days)?;
        let jwt = Jwt::new(&config.cookie_key.0);

        info!("generate token for {} with {} days", self.user, self.days);
        let token = jwt.sign_by_duration(NAME, &self.user, Log::SHOW, ttl)?;
        println!("{token}");
        Ok(())
    }
}
