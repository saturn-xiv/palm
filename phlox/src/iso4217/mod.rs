use std::num::ParseIntError;
use std::result::Result as StdResult;

use quick_xml::de::{DeError, from_str};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ISO4217 {
    #[serde(rename = "@Pblshd")]
    pub published: String,
    #[serde(rename = "CcyTbl")]
    pub table: CcyTbl,
}

impl ISO4217 {
    pub fn new() -> StdResult<Self, DeError> {
        from_str::<Self>(LIST_ONE)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcyTbl {
    #[serde(rename = "CcyNtry")]
    pub items: Vec<CcyNtry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcyNtry {
    #[serde(rename = "CtryNm")]
    pub country: Text,
    #[serde(rename = "CcyNm")]
    pub name: CcyNm,
    #[serde(rename = "Ccy")]
    pub code: Option<Text>,
    #[serde(rename = "CcyNbr")]
    pub number: Option<CcyNbr>,
    #[serde(rename = "CcyMnrUnts")]
    pub units: Option<Text>,
}

impl CcyNtry {
    pub fn units(&self) -> StdResult<Option<u8>, ParseIntError> {
        if let Some(ref it) = self.units {
            if it.value != "N.A." {
                return it.value.parse::<u8>().map(Some);
            }
        }
        Ok(None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcyNm {
    #[serde(rename = "@IsFund")]
    pub fund: Option<bool>,
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcyNbr {
    #[serde(rename = "$text")]
    pub value: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "$text")]
    pub value: String,
}

// https://www.iso.org/iso-4217-currency-codes.html
const LIST_ONE: &str = include_str!("list-one.xml");
