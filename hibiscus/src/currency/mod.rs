use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::Result;

// https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list_one.xml
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub country: String,
    pub code: String,
    pub name: String,
    pub id: i32,
    pub fund: bool,
    pub unit: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Iso4217 {
    pub items: Vec<Currency>,
    pub published_at: NaiveDate,
}

impl Iso4217 {
    pub fn new() -> Result<Self> {
        // TODO
        todo!()
    }
}
