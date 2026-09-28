use std::path::Path;

use axum::http::HeaderMap;
use hyper::StatusCode;
use portal::{
    HttpError, Result, models::user::email::Dao as EmailUserDao, orm::postgresql::Connection as Db,
    queue::rabbitmq::Client as RabbitMq,
};

use super::{
    WebHook,
    models::{
        github::hooks::{Header as GithubHeader, requests as github_requests},
        gogs::hooks::{Header as GogsHeader, requests as gogs_requests},
        job::Item as Job,
    },
};

impl WebHook {
    pub async fn execute<P: AsRef<Path>>(
        &self,
        db: &mut Db,
        queue: &RabbitMq,
        ip: &str,
        jobs_dir: P,
        name: &str,
        (headers, body): (&HeaderMap, &str),
    ) -> Result<()> {
        let (email, args) = match self {
            Self::Gogs { email, secret } => {
                let header = GogsHeader::new(headers);
                log::debug!("receive gogs hook request: {:?}\n{}", header, body);
                header.verify(secret, body)?;

                (
                    email,
                    match header.event.as_str() {
                        "push" => {
                            let req: gogs_requests::push::Item = serde_json::from_str(body)?;
                            Ok(vec![
                                header.delivery,
                                header.event,
                                req.r#ref,
                                req.before,
                                req.after,
                            ])
                        }
                        ev => Err(Box::new(HttpError(
                            StatusCode::BAD_REQUEST,
                            Some(format!("unsupported gogs event {ev}")),
                        ))),
                    },
                )
            }
            Self::Github { email, secret } => {
                let header = GithubHeader::new(headers);
                log::debug!("receive github hook request: {:?}\n{}", header, body);
                header.verify(secret, body)?;

                (
                    email,
                    match header.event.as_str() {
                        "ping" => {
                            let req: github_requests::ping::Item = serde_json::from_str(body)?;
                            Ok(vec![header.delivery, header.event, req.zen])
                        }
                        "push" => {
                            let req: github_requests::push::Item = serde_json::from_str(body)?;
                            Ok(vec![
                                header.delivery,
                                header.event,
                                req.r#ref,
                                req.before,
                                req.after,
                            ])
                        }
                        ev => Err(Box::new(HttpError(
                            StatusCode::BAD_REQUEST,
                            Some(format!("unsupported github event {ev}")),
                        ))),
                    },
                )
            }
        };

        let args = args?;
        let email_user = EmailUserDao::by_email(db, email)?;
        Job::publish(
            db,
            queue,
            ip,
            (email_user.user_id, email),
            (jobs_dir, name, args),
        )
        .await?;
        Ok(())
    }
}
