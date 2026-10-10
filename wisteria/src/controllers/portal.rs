use std::ops::{Deref, DerefMut};
use std::result::Result as StdResult;

use axum::{
    Extension,
    body::Body,
    extract::{Json, Multipart, Path, Query},
    http::HeaderMap,
    http::Response,
    response::Html,
};
use axum_extra::extract::cookie::CookieJar;
use hyper::StatusCode;
use portal::{
    HtmlResult, JsonResult,
    controllers::{attachments as attachment_api, home as home_},
    graphql::{Session, Succeeded},
    web_try,
    wechat::webhook::{
        EncryptRequest as WechatWebHookEncryptRequest, Query as WechatWebHookQuery,
        Request as WechatWebHookRequest, Response as WechatWebHookResponse,
        ping::Query as WechatWebHookPingQuery,
    },
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

// https://developers.weixin.qq.com/apiExplorer?type=messagePush
#[axum::debug_handler]
pub async fn wechat_webhook_ping(
    Extension(state): Extension<State>,
    Query(query): Query<WechatWebHookPingQuery>,
) -> (StatusCode, String) {
    match query.verify(&state.wechat_web_hook.token) {
        Ok(_) => (StatusCode::OK, query.echo_str),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()),
    }
}

#[axum::debug_handler]
pub async fn wechat_oauth2_webhook(
    Extension(state): Extension<State>,
    Query(query): Query<WechatWebHookQuery>,
    Json(body): Json<WechatWebHookEncryptRequest>,
) -> JsonResult<WechatWebHookResponse> {
    log::info!("receive message from user {}", body.to_user_name);
    web_try!(query.verify(&state.wechat_web_hook.token, &body.encrypt));
    let it = match web_try!(
        state
            .wechat_web_hook
            .request(&state.wechat_oauth2.app_id, &body.encrypt)
    ) {
        WechatWebHookRequest::AuthorizationChange(ref it) => {
            log::debug!("{:?}", it);
            // TODO
            web_try!(
                state
                    .wechat_web_hook
                    .response(&state.wechat_oauth2.app_id, &Succeeded::default(),)
            )
        }
    };

    Ok(Json(it))
}
#[axum::debug_handler]
pub async fn wechat_mini_program_webhook(
    Extension(state): Extension<State>,
    Query(query): Query<WechatWebHookQuery>,
    Json(body): Json<WechatWebHookEncryptRequest>,
) -> JsonResult<WechatWebHookResponse> {
    log::info!("receive message from user {}", body.to_user_name);
    web_try!(query.verify(&state.wechat_web_hook.token, &body.encrypt));
    let it = match web_try!(
        state
            .wechat_web_hook
            .request(&state.wechat_mini_program.app_id, &body.encrypt)
    ) {
        WechatWebHookRequest::AuthorizationChange(ref it) => {
            log::debug!("{:?}", it);
            // TODO
            web_try!(
                state
                    .wechat_web_hook
                    .response(&state.wechat_mini_program.app_id, &Succeeded::default(),)
            )
        }
    };

    Ok(Json(it))
}
