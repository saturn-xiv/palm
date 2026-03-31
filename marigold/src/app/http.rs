use std::path::Path;

use clap::ValueEnum;
use phlox::Result;
use strum::{Display as EnumDisplay, EnumString};

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumDisplay, EnumString, ValueEnum, Default)]
pub enum Theme {
    #[strum(serialize = "bulma")]
    Bulma,
    #[default]
    #[strum(serialize = "bootstrap")]
    Bootstrap,
}
// impl fmt::Display for Theme {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

pub async fn start<P: AsRef<Path>>(_config: P, _port: u16, _theme: Theme) -> Result<()> {
    // TODO
    Ok(())
}
