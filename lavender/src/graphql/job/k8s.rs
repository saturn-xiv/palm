use hyper::StatusCode;
use portal::{
    HttpError, Jwt, Result, cache::redis::StandaloneConnection as Cache, graphql::Session,
    orm::postgresql::Connection as Db, rbac::Rbac, shell,
};

use super::super::ROLE as OPERATOR;

// kubectl create token headlamp-admin --duration 8h -n kube-system

pub async fn generate_headlamp_token<R: Rbac, J: Jwt>(
    ss: &Session,
    db: &mut Db,
    cache: &mut Cache,
    rbac: &R,
    jwt: &J,
    hours: u16,
) -> Result<String> {
    if !(1..=8).contains(&hours) {
        return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
    }
    let current_user = ss.current_user(db, cache, jwt).await?;
    rbac.has_role(current_user.id(), OPERATOR).await?;
    let hours = format!("{}h", hours);
    let stdout = shell(
        "/tmp",
        "/usr/bin/kubectl",
        vec![
            "create",
            "token",
            "headlamp-admin",
            "--duration",
            hours.as_str(),
            "-n",
            "kube-system",
        ],
    )?;
    Ok(stdout)
}
