use std::ops::{Deref, DerefMut};
use std::result::Result as StdResult;

use axum::{
    Extension,
    body::Body,
    extract::{Json, Multipart, Path, Query},
    http::Response,
    http::{HeaderMap, StatusCode},
    response::Html,
};
use axum_extra::extract::cookie::CookieJar;
use portal::{
    HtmlResult, JsonResult,
    controllers::{attachments as attachment_api, home as home_},
    graphql::{Session, Succeeded},
    web_try,
};
use serde::Deserialize;

use super::super::graphql::context::State;

#[axum::debug_handler]
pub async fn home(
    Extension(state): Extension<State>,
    headers: HeaderMap,
    jar: CookieJar,
) -> HtmlResult {
    let state = state.deref();
    let mut db = web_try!(state.db.get());
    let db = db.deref_mut();
    let mut cache = web_try!(state.cache.get());
    let body = web_try!(home_(&Session::new(&headers, &jar), db, &mut cache).await);
    Ok(Html(body))
}

#[derive(Deserialize)]
pub struct AttachmentUploadQuery {
    pub public: Option<bool>,
}
#[axum::debug_handler]
pub async fn attachments_upload(
    headers: HeaderMap,
    jar: CookieJar,
    Extension(state): Extension<State>,
    Query(query): Query<AttachmentUploadQuery>,
    multipart: Multipart,
) -> JsonResult<Succeeded> {
    let state = state.deref();
    let mut db = web_try!(state.db.get());
    let db = db.deref_mut();
    let mut cache = web_try!(state.cache.get());

    web_try!(
        attachment_api::upload(
            &Session::new(&headers, &jar),
            db,
            &mut cache,
            &state.loquat,
            &state.s3,
            multipart,
            query.public.unwrap_or(false)
        )
        .await
    );

    Ok(Json(Succeeded::default()))
}

#[derive(Deserialize)]
pub struct AttachmentShowQuery {
    pub download: Option<bool>,
}
#[axum::debug_handler]
pub async fn attachments_show(
    Extension(state): Extension<State>,
    Path((token, uid)): Path<(String, String)>,
    Query(query): Query<AttachmentShowQuery>,
) -> StdResult<Response<Body>, (StatusCode, String)> {
    let state = state.deref();
    let mut db = web_try!(state.db.get());
    let db = db.deref_mut();
    let it = web_try!(
        attachment_api::show(
            db,
            &state.loquat,
            &state.s3,
            &token,
            &uid,
            query.download.unwrap_or(false)
        )
        .await
    );
    Ok(it)
}
