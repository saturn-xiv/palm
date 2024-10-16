pub mod x_corporation;

use std::path::Path;

use actix_web::web;
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

#[cfg(not(debug_assertions))]
pub fn register<P>(_themes: P) -> impl FnOnce(&mut web::ServiceConfig)
where
    P: AsRef<Path>,
{
    |_config: &mut web::ServiceConfig| {}
}

#[cfg(debug_assertions)]
pub fn register<P>(themes: P) -> impl FnOnce(&mut web::ServiceConfig)
where
    P: AsRef<Path>,
{
    move |config: &mut web::ServiceConfig| {
        let themes = themes.as_ref();
        {
            let root = themes.join("x-corporation-uiCookies").join("HTML");
            let base = "/themes/x-corporation";
            config
                .service(
                    actix_files::Files::new(&format!("{base}/img"), root.join("img"))
                        .prefer_utf8(true)
                        .use_etag(true)
                        .use_last_modified(true),
                )
                .service(
                    actix_files::Files::new(&format!("{base}/fonts"), root.join("fonts"))
                        .prefer_utf8(true)
                        .use_etag(true)
                        .use_last_modified(true),
                )
                .service(
                    actix_files::Files::new(&format!("{base}/css"), root.join("css"))
                        .prefer_utf8(true)
                        .use_etag(true)
                        .use_last_modified(true),
                )
                .service(
                    actix_files::Files::new(&format!("{base}/js"), root.join("js"))
                        .prefer_utf8(true)
                        .use_etag(true)
                        .use_last_modified(true),
                );
        }
    }
}

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
pub enum Theme {
    #[default]
    #[strum(serialize = "x-corporation")]
    #[serde(rename = "x-corporation")]
    XCorporation,
}

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
