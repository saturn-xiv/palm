use std::path::PathBuf;
use std::{collections::HashSet, ops::Deref};

use actix_web::{get, web, HttpResponse, Responder, Result as WebResult};
use chrono::NaiveDate;
use log::info;
use palm::{
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    try_web, Result,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::super::{models::log::Item as Log, NAME};

#[derive(Debug, Clone)]
pub struct Journal(pub Option<PathBuf>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filter {
    pub unit: Option<String>,
    pub since: NaiveDate,
    pub until: NaiveDate,
}

impl Filter {
    pub const DATE_FORMAT: &'static str = "%Y-%m-%d";
}

#[get("/{token}")]
pub async fn index(
    jwt: web::Data<Jwt>,
    journal: web::Data<Journal>,
    params: web::Path<String>,
    filter: web::Query<Filter>,
) -> WebResult<impl Responder> {
    let filter = filter.into_inner();
    let journal = journal.into_inner();
    let journal = journal.deref();
    let token = params.into_inner();
    let (_, user) = try_web!(jwt.verify(&token, NAME, Log::SHOW))?;
    info!("query by user({user}): {:?}", filter);

    let body = try_web!(Response::new(journal.0.clone(), &filter))?;

    Ok(HttpResponse::Ok().json(body))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub items: Vec<Log>,
    pub filter: Filter,
    pub systemd_units: HashSet<String>,
}

impl Response {
    pub fn new<P: AsRef<Path> + Clone>(dir: Option<P>, filter: &Filter) -> Result<Self> {
        let it = Self {
            items: Log::load(dir.clone(), filter.unit.clone(), filter.since, filter.until)?,
            systemd_units: Log::systemd_units(dir.clone())?,
            filter: filter.clone(),
        };
        Ok(it)
    }
}
