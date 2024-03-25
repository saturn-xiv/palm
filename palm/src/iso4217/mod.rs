// https://www.iso.org/iso-4217-currency-codes.html

use chrono::NaiveDate;
use quick_xml::de::from_str as xml_from_str;
use serde::{Deserialize, Serialize};

use super::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename = "ISO_4217")]
pub struct ListOne {
    #[serde(rename = "@Pblshd")]
    pub pblshd: NaiveDate,
    pub ccytbl: CcyTbl,
}

impl ListOne {
    const XML: &'static str = include_str!("list-one.xml");
    pub fn new() -> Result<Self> {
        let it = xml_from_str(Self::XML)?;
        Ok(it)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "CcyTbl")]
pub struct CcyTbl {
    pub ccyntry: Vec<CcyNtry>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "CcyNtry")]
pub struct CcyNtry {
    pub ctrynm: CtryNm,
    pub ccynm: CcyNm,
    pub ccy: Ccy,
    pub ccynbr: CcyNbr,
    pub ccymnrunts: CcyMnrUnts,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "CtryNm")]
pub struct CtryNm {
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "CcyNm")]
pub struct CcyNm {
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "Ccy")]
pub struct Ccy {
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "CcyNbr")]
pub struct CcyNbr {
    #[serde(rename = "$text")]
    pub value: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "CcyMnrUnts")]
pub struct CcyMnrUnts {
    #[serde(rename = "$text")]
    pub value: u8,
}
