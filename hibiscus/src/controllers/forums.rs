use actix_web::{error::ErrorInternalServerError, get, web, Responder, Result as WebResult};
use petunia::orm::postgresql::Pool as DbPool;

#[get("/")]
pub async fn index(db: web::Data<DbPool>) -> WebResult<impl Responder> {
    let db = db.into_inner();
    let mut _db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    // TODO
    Ok(web::Html::new("index forum"))
}

#[get("/{code}")]
pub async fn by_code(
    db: web::Data<DbPool>,
    params: web::Path<(String,)>,
) -> WebResult<impl Responder> {
    let (_code,) = params.into_inner();

    let db = db.into_inner();
    let mut _db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    Ok(web::Html::new("show forum by code"))
}
