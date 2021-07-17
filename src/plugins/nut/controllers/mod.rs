pub mod attachments;

use std::ops::Deref;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{
    get,
    http::header::{ContentEncoding, CONTENT_ENCODING},
    web, HttpResponse, Responder, Result,
};
use askama::Template;
use mime::{TEXT_HTML_UTF_8, TEXT_PLAIN_UTF_8, TEXT_XML};

use super::super::super::{
    cache::redis::Pool as CachePool,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    request::Locale,
    Error, HttpResult,
};

#[derive(Template)]
#[template(path = "themes/bootstrap/home.html")]
pub struct Home {
    pub language: String,
}

async fn index(lang: &str, _db: &Db) -> HttpResult<impl Responder> {
    // TODO
    let tpl = Home {
        language: lang.to_string(),
    };
    let it = HttpResponse::Ok()
        .content_type(TEXT_HTML_UTF_8)
        .body(tpl.render().map_err(Error::from)?);
    Ok(it)
}

#[get("/")]
pub async fn home(
    locale: Locale,
    db: web::Data<DbPool>,
    _cache: web::Data<CachePool>,
) -> HttpResult<impl Responder> {
    let db = db.get().map_err(Error::from)?;
    let db = db.deref();
    index(&locale.0, db).await
}

#[get("/{lang}/")]
pub async fn home_by_lang(
    locale: web::Path<(String,)>,
    db: web::Data<DbPool>,
    _cache: web::Data<CachePool>,
) -> HttpResult<impl Responder> {
    let db = db.get().map_err(Error::from)?;
    let db = db.deref();
    index(&locale.0, db).await
}

#[derive(Template)]
#[template(path = "robots.txt", escape = "none")]
pub struct RobotsTxt<'a> {
    pub domain: &'a str,
}

// https://developers.google.com/search/docs/advanced/robots/create-robots-txt
#[get("/robots.txt")]
pub async fn robots_txt(
    _db: web::Data<DbPool>,
    _cache: web::Data<CachePool>,
) -> HttpResult<impl Responder> {
    // TODO
    let tpl = RobotsTxt { domain: "todo!" };
    let it = HttpResponse::Ok()
        .content_type(TEXT_PLAIN_UTF_8)
        .body(tpl.render().map_err(Error::from)?);
    Ok(it)
}

// https://developers.google.com/search/docs/advanced/sitemaps/build-sitemap#xml
#[get("/sitemap.xml.gz")]
pub async fn sitemap_xml_gz(
    _db: web::Data<DbPool>,
    _cache: web::Data<CachePool>,
) -> HttpResult<impl Responder> {
    // TODO
    let it = HttpResponse::Ok()
        .content_type(TEXT_XML)
        .insert_header((CONTENT_ENCODING, ContentEncoding::Gzip))
        .body("todo!");
    Ok(it)
}

// https://cyber.harvard.edu/rss/rss.html
#[get("/{lang}/rss.xml")]
pub async fn rss_xml(
    _locale: web::Path<(String,)>,
    _db: web::Data<DbPool>,
    _cache: web::Data<CachePool>,
) -> HttpResult<impl Responder> {
    // TODO
    let it = HttpResponse::Ok().content_type(TEXT_XML).body("todo!");
    Ok(it)
}

#[get("/3rd/{file:.*}")]
pub async fn third(file: web::Path<(String,)>) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("node_modules").join(&file.0))?)
}
