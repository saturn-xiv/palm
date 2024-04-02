use actix_web::{
    get, http::header::ContentType, web, HttpResponse, Responder, Result as WebResult,
};
use askama::Template;
use palm::{
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    try_web, Result,
};

use super::super::{models::log::Item as Log, NAME};

#[get("/{token}/")]
pub async fn index(jwt: web::Data<Jwt>, params: web::Path<String>) -> WebResult<impl Responder> {
    let token = params.into_inner();
    try_web!(jwt.verify(&token, NAME, Log::SHOW))?;

    let tpl = {
        let mut it = try_web!(Show::new())?;
        try_web!(it.set_archives())?;
        it
    };
    let body = try_web!(tpl.render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[derive(Template)]
#[template(path = "logs/show.html")]
pub struct Show {}

impl Show {
    pub fn new() -> Result<Self> {
        todo!()
    }

    pub fn set_archives(&mut self) -> Result<()> {
        todo!()
    }
}
