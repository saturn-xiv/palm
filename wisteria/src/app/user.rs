use std::path::Path;

use portal::{Result, parse_toml};
use serde::{Deserialize, Serialize};

pub fn list<P: AsRef<Path>>(_config: P) -> Result<()> {
    // TODO
    Ok(())
}

pub fn create_by_email<P: AsRef<Path>>(
    config: P,
    _name: &str,
    _email: &str,
    _password: &str,
) -> Result<()> {
    let _config: Config = parse_toml(config)?;
    // TODO
    Ok(())
}
pub async fn add_role<P: AsRef<Path>>(_config: P, _user: &str, _role: &str) -> Result<()> {
    // TODO
    Ok(())
}

pub async fn delete_role<P: AsRef<Path>>(_config: P, _user: &str, _role: &str) -> Result<()> {
    // TODO
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {}
