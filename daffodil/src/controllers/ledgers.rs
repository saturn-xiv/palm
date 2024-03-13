use std::ops::DerefMut;

use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder, Result};
use askama::Template;
use camelia::orm::postgresql::Pool as DbPool;
use palm::{
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    try_web,
};

use super::super::{
    graphql::ledger::ExportRequest,
    models::{
        bill::{Dao as BillDao, Item as Bill},
        ledger::{Dao as LedgerDao, Item as Ledger},
    },
    NAME,
};

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
    let ledger = try_web!(LedgerDao::by_uid(db, &uid))?;
    let bills = try_web!(BillDao::by_ledger(db, ledger.id))?;

    let body = try_web!(Show {
        ledger: &ledger,
        bills: &bills
    }
    .render())?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[derive(Template)]
#[template(path = "ledgers/show.html")]
pub struct Show<'a> {
    pub ledger: &'a Ledger,
    pub bills: &'a [Bill],
}
