use std::path::Path;

use petunia::{
    check_config_permission,
    crypto::Key,
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    parser::from_toml,
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(clap::Parser, PartialEq, Eq, Debug)]
pub struct Command {
    #[clap(short, long)]
    pub user: String,
    #[clap(short, long, default_value = "10")]
    pub years: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub secrets: Key,
}
impl Command {
    pub async fn launch<P: AsRef<Path>>(&self, config: P) -> Result<()> {
        let config: Config = {
            let config = config.as_ref();
            log::info!("load config from {}", config.display());
            check_config_permission(config)?;
            from_toml(config)?
        };
        log::info!(
            "generate a {}-years token for user({})",
            self.years,
            self.user
        );
        let jwt = Jwt::new(config.secrets.0.clone());
        let (nbf, exp) = Jwt::years(self.years as i32)?;
        let token = jwt.sign(&self.user, super::NAME, nbf, exp)?;
        println!("{token}");
        Ok(())
    }
}
