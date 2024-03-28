pub mod view;

use std::ops::{Deref, DerefMut};

use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder, Result};
use askama::Template;
use camelia::orm::postgresql::Pool as DbPool;
use palm::{
    cache::redis::Pool as CachePool,
    crypto::aes::Aes,
    duration_from_days,
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    minio::Client as Minio,
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
    s3: web::Data<Minio>,
    token: web::Path<(String, String)>,
) -> Result<impl Responder> {
    let (lang, token) = token.into_inner();
    let (_, uid) = try_web!(jwt.verify(&token, NAME, LedgerItem::SHOW))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let aes = aes.deref();
    let aes = aes.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();

    let ttl = try_web!(duration_from_days(7))?;
    let item = try_web!(LedgerDao::by_uid(db, &uid))?;
    let tpl = try_web!(view::Show::new(db, ch, s3, aes, &lang, &item, ttl).await)?;
    let body = try_web!(tpl.render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}
