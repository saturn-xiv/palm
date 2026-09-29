pub mod job;
pub mod logging;
pub mod monitoring;

use portal::{Result, rbac::Rbac};

pub const ROLE: &str = "lavender.operator";

pub async fn can<R: Rbac>(rbac: &R, user: i64) -> Result<()> {
    rbac.has(user, ROLE).await
}
