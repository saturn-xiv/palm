use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Template)]
#[template(path = "views/x-corporation/services.html.j2")]
pub struct View {}
