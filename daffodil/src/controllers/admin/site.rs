use std::ops::{Deref, DerefMut};

use actix_multipart::form::MultipartForm;
use actix_web::{post, web, Responder, Result as WebResult};
use casbin::Enforcer;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, s3::Client as S3,
    session::Session, try_web,
};
use tokio::sync::Mutex;

use super::super::super::{controllers::attachments::UploadForm, session::current_user, NAME};

#[post("/favicon")]
pub async fn favicon(
    ss: Session,
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    s3: web::Data<S3>,
    enforcer: web::Data<Mutex<Enforcer>>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> WebResult<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let jwt = jwt.deref();
    let s3 = s3.deref();

    {
        let mut db = try_web!(db.get())?;
        let db = db.deref_mut();

        {
            let (_, user) = try_web!(current_user(&ss, db, jwt))?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            try_web!(user.is_administrator(enf))?;
        }
    }

    let it = try_web!(form.save::<Favicon>(&ss, db, jwt, s3, NAME).await)?;
    Ok(web::Json(it))
}

pub struct Favicon;
