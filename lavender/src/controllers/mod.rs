use std::path::Path;

use axum::http::HeaderMap;
use portal::{
    Result, models::user::email::Dao as EmailUserDao, orm::postgresql::Connection as Db,
    queue::rabbitmq::Client as RabbitMq,
};

use super::models::{
    gogs::{hooks::Header as GogsHeader, hooks::requests::push::Item as GogsPushRequest},
    job::Item as Job,
};

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
                let header = GogsHeader::new(headers);
                log::debug!(
                    "receive gogs hook request: {} {} {}\n{}",
                    header.delivery,
                    header.event,
                    header.signature,
                    body
                );
                header.verify(secret, body)?;
                let _: GogsPushRequest = serde_json::from_str(body)?;

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
