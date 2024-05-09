use chrono::NaiveDate;
use log::error;
use quick_xml::de::from_str as xml_from_str;
use serde::{Deserialize, Serialize};

use super::super::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: Option<i32>,
    pub country: String,
    pub name: String,
    pub code: Option<String>,
    pub unit: Option<i32>,
}

impl Item {
    pub fn all() -> Result<Vec<Self>> {
        let items = xml_from_str::<ListOne>(include_str!("list-one.xml"))?
            .ccytbl
            .ccyntry
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(items)
    }
}

impl From<CcyNtry> for Item {
    fn from(x: CcyNtry) -> Self {
        Self {
            id: x.ccynbr.map(|x| x.value as i32),
            country: x.ctrynm.value.clone(),
            name: x.ccynm.value.clone(),
            code: x.ccy.map(|x| x.value.clone()),
            unit: match x.ccymnrunts {
                Some(ref x) => x.value().map(|x| x as i32),
                None => None,
            },
        }
    }
}

// impl CcyNtry {
//     pub fn to_currency(&self) -> Option<super::Currency> {
//         if let Some(ref id) = self.ccynbr {
//             if let Some(ref code) = self.ccy {
//                 if let Some(ref unit) = self.ccymnrunts {
//                     if let Some(unit) = unit.value() {
//                         let it = super::Currency {
//                             id: id.value as i32,
//                             name: self.ccynm.value.clone(),
//                             country: self.ctrynm.value.clone(),
//                             code: code.value.clone(),
//                             unit: unit as i32,
//                         };
//                         return Some(it);
//                     }
//                 }
//             }
//         }
//         // log::debug!("invalid currency {:?}",self);
//         None
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "ISO_4217")]
pub struct ListOne {
    #[serde(rename = "@Pblshd")]
    pub pblshd: NaiveDate,
    #[serde(rename = "CcyTbl")]
    pub ccytbl: CcyTbl,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CcyTbl {
    #[serde(rename = "CcyNtry")]
    pub ccyntry: Vec<CcyNtry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CcyNtry {
    #[serde(rename = "CtryNm")]
    pub ctrynm: CtryNm,
    #[serde(rename = "CcyNm")]
    pub ccynm: CcyNm,
    #[serde(rename = "Ccy")]
    pub ccy: Option<Ccy>,
    #[serde(rename = "CcyNbr")]
    pub ccynbr: Option<CcyNbr>,
    #[serde(rename = "CcyMnrUnts")]
    pub ccymnrunts: Option<CcyMnrUnts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CtryNm {
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CcyNm {
    #[serde(rename = "@IsFund")]
    pub fund: Option<bool>,
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ccy {
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CcyNbr {
    #[serde(rename = "$text")]
    pub value: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CcyMnrUnts {
    #[serde(rename = "$text")]
    pub value: String,
}

impl CcyMnrUnts {
    pub fn value(&self) -> Option<u8> {
        if self.value == "N.A." {
            return None;
        }
        match self.value.parse() {
            Ok(v) => Some(v),
            Err(e) => {
                error!("bad CcyMnrUnts {}: {:?}", self.value, e);
                None
            }
        }
    }
}
