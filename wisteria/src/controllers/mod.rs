pub mod cms;

use std::ops::{Deref, DerefMut};

use axum::{
    Extension,
    http::{HeaderMap, StatusCode},
    response::Html,
};
use axum_extra::extract::cookie::CookieJar;
use portal::{HtmlResult, controllers::home as home_, graphql::Session, web_try};

use super::graphql::context::State;

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
