use actix_web::{get, web, Responder, Result as WebResult};
use chrono::NaiveDateTime;
use petunia::Result;

#[get("/{uid}/{year}-{month}-{day}")]
pub async fn by_year_month_day(
    _params: web::Path<(String, i32, i32, i32)>,
) -> WebResult<impl Responder> {
    // TODO
    Ok(web::Html::new("<h1>ledgers by year month day</h1>"))
}
#[get("/{uid}/{year}-{month}")]
pub async fn by_year_month(_params: web::Path<(String, i32, i32)>) -> WebResult<impl Responder> {
    // TODO
    Ok(web::Html::new("<h1>ledgers by year month</h1>"))
}

#[get("/{uid}/{year}")]
pub async fn by_year(_params: web::Path<(String, i32)>) -> WebResult<impl Responder> {
    // TODO
    Ok(web::Html::new("<h1>ledgers by year month</h1>"))
}

async fn render(token: &str, begin: NaiveDateTime, end: NaiveDateTime) -> Result<String> {
    todo!()
}
