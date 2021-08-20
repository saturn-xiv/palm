use std::ops::Deref;

use actix_web::{post, web, HttpResponse, Responder};

use super::super::super::super::{
    orm::postgresql::Pool as Db,
    twilio::sms::{DeliveryStatusCallbackForm, IncomingMessagesCallbackForm},
    Error, HttpResult,
};
use super::super::models::log::Dao as LogDao;

#[post("/incoming-messages")]
pub async fn incoming_messages(
    form: web::Form<IncomingMessagesCallbackForm>,
    db: web::Data<Db>,
) -> HttpResult<impl Responder> {
    let db = db.get().map_err(Error::from)?;
    let db = db.deref();
    let form = form.deref();
    info!("{:?}", form);
    LogDao::add(
        db,
        &form.from,
        &form.to,
        &serde_json::to_string(form).map_err(Error::from)?,
    )?;
    Ok(HttpResponse::Ok().finish())
}

#[post("/delivery-status")]
pub async fn delivery_status(
    form: web::Form<DeliveryStatusCallbackForm>,
    db: web::Data<Db>,
) -> HttpResult<impl Responder> {
    let db = db.get().map_err(Error::from)?;
    let _db = db.deref();
    let form = form.deref();
    info!("{:?}", form);
    // LogDao::add(form.from, form.respond_to(_: &HttpRequest), body: &str)
    Ok(HttpResponse::Ok().finish())
}
