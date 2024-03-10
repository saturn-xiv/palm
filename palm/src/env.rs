use std::default::Default;

use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

#[derive(Serialize, Deserialize, EnumDisplay, EnumString, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Environment {
    Production,
    Staging,
    #[default]
    Development,
    Test,
}
