use std::fs;
use std::io::read_to_string;
use std::path::Path;

use serde::de::DeserializeOwned;

use super::Result;

pub fn from_toml<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> Result<T> {
    let file = fs::File::open(file)?;
    let buf = read_to_string(file)?;
    let it = toml::from_str(&buf)?;
    Ok(it)
}
