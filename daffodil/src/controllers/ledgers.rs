use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder, Result};
use askama::Template;
use camelia::{
    controllers::i18n as tpl_i18n, graphql::site::InfoResponse as SiteInfo,
    orm::postgresql::Pool as DbPool,
};
use palm::{
    cache::redis::Pool as CachePool,
    crypto::aes::Aes,
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    try_web,
};

use super::super::{
    models::{
        bill::{Dao as BillDao, Item as Bill},
        ledger::{Dao as LedgerDao, Item as Ledger},
    },
    NAME,
};

#[get("/{token}")]
pub async fn show(
    db: web::Data<DbPool>,
    ch: web::Data<CachePool>,
    jwt: web::Data<Jwt>,
    aes: web::Data<Aes>,
    token: web::Path<(String, String)>,
) -> Result<impl Responder> {
    let (lang, token) = token.into_inner();
    let (_, uid) = try_web!(jwt.verify(&token, NAME, Ledger::SHOW))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let aes = aes.deref();

    let ledger = try_web!(LedgerDao::by_uid(db, &uid))?;
    let bills = try_web!(BillDao::by_ledger(db, ledger.id))?;

    let body = try_web!(Show {
        ledger,
        bills,
        site: try_web!(SiteInfo::new(db, ch, aes, &lang))?,
        i18n: try_web!(tpl_i18n(db, &lang))?
    }
    .render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[derive(Template)]
#[template(path = "ledgers/show.html")]
pub struct Show {
    pub ledger: Ledger,
    pub bills: Vec<Bill>,
    pub site: SiteInfo,
    pub i18n: HashMap<String, String>,
}
