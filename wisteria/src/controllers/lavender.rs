use std::ops::{Deref, DerefMut};

use axum::{
    Extension,
    extract::{Json, Path},
    http::{HeaderMap, StatusCode},
};
use portal::{
    JsonResult,
    graphql::{Session, Succeeded},
    web_try,
};

use super::super::graphql::context::State;

#[axum::debug_handler]
pub async fn gogs_web_hook(
    Extension(state): Extension<State>,
    Path(name): Path<String>,
    headers: HeaderMap,
    body: String,
) -> JsonResult<Succeeded> {
    let state = state.deref();
    let mut db = web_try!(state.db.get());
    let db = db.deref_mut();

    let hook = state.lavender.web_hooks.get(&name).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            format!("couldn't found job {name}"),
        )
    })?;
    let ip = Session::detect_client_ip(&headers).unwrap_or_default();
    web_try!(
        hook.execute(
            db,
            &state.queue,
            &ip,
            &state.lavender.jobs_dir,
            &name,
            (&headers, &body)
        )
        .await
    );

    Ok(Json(Succeeded::default()))
}
