use std::ops::{Deref, DerefMut};

use actix_multipart::form::MultipartForm;
use actix_web::{error::ErrorBadRequest, post, web, Responder, Result as WebResult};
use casbin::Enforcer;
use daffodil::{controllers::attachments::UploadForm, session::current_user};
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, s3::Client as S3,
    session::Session, try_web,
};
use tokio::sync::Mutex;

use super::super::{
    models::{
        entry::{Dao as EntryDao, Item as Entry},
        ledger::Dao as LedgerDao,
    },
    NAME,
};

#[post("/bills-upload")]
pub async fn bills_upload(
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
    let enf = enforcer.deref();

    {
        let mut db = try_web!(db.get())?;
        let db = db.deref_mut();
        let (_, user) = try_web!(current_user(&ss, db, jwt))?;
        {
            let id = form
                .json
                .resource_id
                .ok_or(ErrorBadRequest("nil accounting entry id"))?;
            let ie = try_web!(EntryDao::by_id(db, id))?;
            let il = try_web!(LedgerDao::by_id(db, ie.ledger_id))?;

            try_web!(il.can_append(&user, enf).await)?;
        }
    }

    let it = try_web!(form.save::<Entry>(&ss, db, jwt, s3, NAME).await)?;
    Ok(web::Json(it))
}
