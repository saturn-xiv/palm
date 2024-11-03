use actix_web::{get, web, Responder, Result as WebResult};

#[get("/")]
pub async fn get() -> WebResult<impl Responder> {
    // TODO
    Ok(web::Html::new("<h1>home</h1>"))
}
