pub mod list_one;

use serde::{Deserialize, Serialize};

// https://www.iso.org/iso-4217-currency-codes.html
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub name: String,
    pub code: String,
    pub number: String,
    pub country: String,
    pub units: u8,
    pub is_fund: Option<bool>,
}
