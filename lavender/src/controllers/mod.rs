pub mod github;
pub mod gogs;

use std::path::Path;

use axum::http::HeaderMap;
use portal::{
    Result, models::user::email::Dao as EmailUserDao, orm::postgresql::Connection as Db,
    queue::rabbitmq::Client as RabbitMq,
};
use serde_json::from_str as json_from_str;

use super::models::job::Item as Job;

// https://gogs.io/advancing/webhooks
impl super::WebHook {
    pub async fn execute<P: AsRef<Path>>(
        &self,
        db: &mut Db,
        queue: &RabbitMq,
        ip: &str,
        jobs_dir: P,
        name: &str,
        (headers, body): (&HeaderMap, &str),
    ) -> Result<()> {
        match self {
            Self::Gogs { email, secret } => {
                let header = gogs::hooks::Header::new(headers);
                log::debug!(
                    "receive gogs hook request: {} {} {}\n{}",
                    header.delivery,
                    header.event,
                    header.signature,
                    body
                );
                header.verify(secret, body)?;
                let _: gogs::hooks::request::Item = json_from_str(body)?;

                let args = vec![header.delivery, header.event];

                let email_user = EmailUserDao::by_email(db, email)?;
                Job::publish(
                    db,
                    queue,
                    ip,
                    (email_user.user_id, email),
                    (jobs_dir, name, args),
                )
                .await?;
            }
        }

        Ok(())
    }
}
