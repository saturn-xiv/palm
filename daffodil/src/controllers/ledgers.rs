use std::ops::DerefMut;

use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder, Result};
use askama::Template;
use camelia::orm::postgresql::Pool as DbPool;
use palm::{
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    try_web,
};

use super::super::{graphql::ledger::ExportRequest, models::ledger::Dao as LedgerDao, NAME};

#[get("/{token}")]
pub async fn show(
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    token: web::Path<String>,
) -> Result<impl Responder> {
    let token = token.into_inner();
    let (_, uid) = try_web!(jwt.verify(&token, NAME, ExportRequest::AUDIENCE))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let item = try_web!(LedgerDao::by_uid(db, &uid))?;

    // TODO
    let body = try_web!(Show { name: &item.name }.render())?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[derive(Template)]
#[template(path = "ledgers/show.html")]
pub struct Show<'a> {
    pub name: &'a str,
}
