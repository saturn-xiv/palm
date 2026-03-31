use std::path::Path;

use phlox::Result;
use serde::{Deserialize, Serialize};

pub async fn seeds<P: AsRef<Path>>(_config: P, _locales: Option<Vec<String>>) -> Result<()> {
    // TODO
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {}
