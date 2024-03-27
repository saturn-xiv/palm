pub mod view;

use std::ops::{Deref, DerefMut};

use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder, Result};
use askama::Template;
use camelia::orm::postgresql::Pool as DbPool;

use palm::{
    cache::redis::Pool as CachePool,
    crypto::aes::Aes,
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    try_web,
};

use super::super::super::{
    models::ledger::{Dao as LedgerDao, Item as LedgerItem},
    NAME,
};

#[get("/{token}")]
pub async fn by_token(
    db: web::Data<DbPool>,
    ch: web::Data<CachePool>,
    jwt: web::Data<Jwt>,
    aes: web::Data<Aes>,
    token: web::Path<(String, String)>,
) -> Result<impl Responder> {
    let (lang, token) = token.into_inner();
    let (_, uid) = try_web!(jwt.verify(&token, NAME, LedgerItem::SHOW))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let aes = aes.deref();

    let item = try_web!(LedgerDao::by_uid(db, &uid))?;
    let tpl = try_web!(view::Show::new(db, ch, aes, &lang, &item))?;
    let body = try_web!(tpl.render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}
