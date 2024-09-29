pub mod attachment;
pub mod category;
pub mod leave_word;
pub mod locale;
pub mod log;
pub mod role;
pub mod session;
pub mod setting;
pub mod tag;
pub mod user;

use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Editor {
    #[default]
    Textarea,
    Slate,
}
