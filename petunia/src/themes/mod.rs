pub mod x_corporation;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub label: String,
    pub link: String,
    pub extra: bool,
    pub children: Vec<Self>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    pub title: String,
    pub subhead: String,
    pub author: Author,
    pub keywords: Vec<String>,
    pub description: String,
    pub copyright: String,
    pub cn_gab: Option<CnGab>,
    pub cn_bi: Option<CnBi>,
    pub cn_icp: Option<CnIcp>,
    pub locale: String,
    pub languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CnGab {
    pub code: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CnIcp {
    pub code: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CnBi {
    pub code: String,
}
