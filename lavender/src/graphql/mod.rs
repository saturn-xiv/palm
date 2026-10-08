pub mod job;
pub mod logging;
pub mod monitoring;

use chrono::{Months, NaiveTime, Utc};
use hyper::StatusCode;
use portal::{
    HttpError, Jwt, Result, cache::redis::StandaloneConnection as Cache, graphql::Session,
    opensearch::Client as Search, orm::postgresql::Connection as Db, rbac::Rbac,
};
use serde_json::json;

pub const ROLE: &str = "lavender.operator";

pub async fn can<R: Rbac>(rbac: &R, user: i64) -> Result<()> {
    rbac.has(user, ROLE).await
}

pub async fn vacuum<R: Rbac, J: Jwt>(
    ss: &Session,
    db: &mut Db,
    cache: &mut Cache,
    rbac: &R,
    jwt: &J,
    search: &Search,
    months: u32,
) -> Result<()> {
    if !(3..=1_200).contains(&months) {
        return Err(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("invalid months".to_string()),
        )));
    }
    let current_user = ss.current_user(db, cache, jwt).await?;
    can(rbac, current_user.id()).await?;
    let since = Utc::now()
        .checked_sub_months(Months::new(months))
        .ok_or_else(|| Box::new(HttpError(StatusCode::BAD_REQUEST, None)))?
        .date_naive()
        .and_time(NaiveTime::MIN);
    let query = json!({
        "query": {
            "range": {
                "created_at": {
                    "lt": since
                }
            }
        }
    });
    search
        .delete_document_by_query::<monitoring::http::Item>(query.clone())
        .await?;
    search
        .delete_document_by_query::<logging::systemd::unit::Item>(query.clone())
        .await?;
    search
        .delete_document_by_query::<logging::kubernetes::pod::Item>(query)
        .await?;
    Ok(())
}
