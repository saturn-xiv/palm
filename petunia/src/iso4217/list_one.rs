use quick_xml::de::from_str;
use serde::Deserialize;

use super::super::Result;

impl super::Currency {
    pub fn list_one() -> Result<Vec<Self>> {
        let mut items = Vec::new();
        for it in Iso4217::new()?.ccytbl.ctrynm.iter() {
            if let Some(ref ccy) = it.ccy {
                if let Some(ref nbr) = it.ccynbr {
                    if let Some(ref unts) = it.ccymnrunts {
                        if unts.value != "N.A." {
                            items.push(Self {
                                name: it.ccynm.value.clone(),
                                country: it.ctrynm.value.clone(),
                                code: ccy.value.clone(),
                                number: nbr.value.clone(),
                                units: unts.value.parse()?,
                            });
                        }
                    }
                }
            }
        }
        Ok(items)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "ISO_4217")]
pub struct Iso4217 {
    #[serde(rename = "CcyTbl")]
    ccytbl: CcyTbl,
    #[serde(rename = "@Pblshd")]
    pblshd: String,
}

impl Iso4217 {
    pub fn new() -> Result<Self> {
        let it = from_str(include_str!("list-one.xml"))?;
        Ok(it)
    }
    pub fn version(&self) -> String {
        self.pblshd.clone()
    }
}

#[derive(Deserialize, Debug, Clone)]
struct CcyTbl {
    #[serde(rename = "CcyNtry")]
    ctrynm: Vec<CcyNtry>,
}
#[derive(Deserialize, Debug, Clone)]
struct CcyNtry {
    #[serde(rename = "CtryNm")]
    ctrynm: CtryNm,
    #[serde(rename = "CcyNm")]
    ccynm: CcyNm,
    #[serde(rename = "Ccy")]
    ccy: Option<Ccy>,
    #[serde(rename = "CcyNbr")]
    ccynbr: Option<CcyNbr>,
    #[serde(rename = "CcyMnrUnts")]
    ccymnrunts: Option<CcyMnrUnts>,
}

#[derive(Deserialize, Debug, Clone)]
struct CtryNm {
    #[serde(rename = "$text")]
    value: String,
}
#[derive(Deserialize, Debug, Clone)]
struct CcyNm {
    #[serde(rename = "$text")]
    value: String,
}
#[derive(Deserialize, Debug, Clone)]
struct Ccy {
    #[serde(rename = "$text")]
    value: String,
}
#[derive(Deserialize, Debug, Clone)]
struct CcyNbr {
    #[serde(rename = "$text")]
    value: String,
}
#[derive(Deserialize, Debug, Clone)]
struct CcyMnrUnts {
    #[serde(rename = "$text")]
    value: String,
}
