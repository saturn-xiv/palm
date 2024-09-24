use petunia::{
    crypto::Key,
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
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

pub fn launch(config: &Config, user: &str, years: u8) -> Result<()> {
    log::info!("generate a {years}-years token for user({user})");
    let jwt = Jwt::new(config.secrets.0.clone());
    let (nbf, exp) = Jwt::years(years as i32)?;
    let token = jwt.sign(user, super::NAME, nbf, exp)?;
    println!("{token}");
    Ok(())
}
