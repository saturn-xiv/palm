use std::path::Path;

use phlox::Result;
use serde::{Deserialize, Serialize};

pub fn list<P: AsRef<Path>>(_config: P) -> Result<()> {
    // TODO
    Ok(())
}

pub fn create_by_email<P: AsRef<Path>>(
    _config: P,
    _name: &str,
    _email: &str,
    _password: &str,
) -> Result<()> {
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
pub struct Config {}
