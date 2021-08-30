use std::sync::Arc;

use chrono::Duration;
use diesel::Connection as DieselConnection;
use mime::{OCTET_STREAM, TEXT_HTML_UTF_8};
use prost::Message as ProstMessage;
use tonic::{Request, Response, Status};
use validator::Validate;

use super::super::super::{
    crypto::Hmac,
    i18n::I18n,
    jwt::Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    queue::amqp::RabbitMq,
    Error, GrpcResult, Result,
};
use super::super::{
    models::{
        log::Dao as LogDao,
        user::{Dao as UserDao, Item as User, Token},
    },
    v1,
};
use super::{EmailField, NameField, PasswordField, Session};

pub struct Service {
    pub hmac: Arc<Hmac>,
    pub jwt: Arc<Jwt>,
    pub db: DbPool,
    pub queue: Arc<RabbitMq>,
}

#[tonic::async_trait]
impl v1::user_server::User for Service {
    async fn sign_in(&self, req: Request<v1::SignInRequest>) -> GrpcResult<v1::SignInResponse> {
        let ss = Session::new(&req);
        let req = req.into_inner();
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        let user = try_grpc!(Self::by_nick_name_or_email(db, &req.user))?;
        Ok(Response::new(v1::SignInResponse::default()))
    }

    async fn sign_up(&self, req: Request<v1::SignUpRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let req = req.into_inner();
        let email = req.email.trim().to_lowercase();
        let nick_name = req.nick_name.trim().to_lowercase();
        let real_name = req.real_name.trim();
        let password = req.password.clone();
        {
            let it = EmailField {
                value: email.clone(),
            };
            try_grpc!(it.validate())?;
        }
        {
            let it = NameField {
                value: nick_name.clone(),
            };
            try_grpc!(it.validate())?;
        }
        {
            let it = NameField {
                value: real_name.to_string(),
            };
            try_grpc!(it.validate())?;
        }
        {
            let it = PasswordField {
                value: password.clone(),
            };
            try_grpc!(it.validate())?;
        }

        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        if UserDao::by_nick_name(db, &nick_name).is_ok() {
            return Err(Status::invalid_argument("Nick name already exists!"));
        }
        if UserDao::by_email(db, &email).is_ok() {
            return Err(Status::invalid_argument("Email already exists!"));
        }

        let hmac = self.hmac.deref();
        let user = try_grpc!(db.transaction::<_, Error, _>(move || {
            UserDao::sign_up(db, hmac, real_name, &nick_name, &email, &password)?;
            let user = UserDao::by_email(db, &email)?;
            LogDao::add(db, user.id, &ss.client_ip(), "Sign up.")?;
            Ok(user)
        }))?;
        Ok(Response::new(()))
    }
    async fn confirm(&self, req: Request<v1::EmailRequest>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }
    async fn confirm_by_token(&self, req: Request<v1::TokenRequest>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }
    async fn unlock(&self, req: Request<v1::EmailRequest>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }
    async fn unlock_by_token(&self, req: Request<v1::TokenRequest>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }
    async fn forgot_password(&self, req: Request<v1::EmailRequest>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }
    async fn reset_password(&self, req: Request<v1::ResetPasswordRequest>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }
    async fn current(&self, req: Request<()>) -> GrpcResult<v1::SignInResponse> {
        Ok(Response::new(v1::SignInResponse::default()))
    }

    async fn change_password(&self, req: Request<v1::ChangePasswordRequest>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }

    async fn logs(&self, req: Request<prost_types::Duration>) -> GrpcResult<v1::IndexLogResponse> {
        Ok(Response::new(v1::IndexLogResponse::default()))
    }

    async fn get_profile(&self, req: Request<()>) -> GrpcResult<v1::UserProfile> {
        Ok(Response::new(v1::UserProfile::default()))
    }
    async fn set_profile(&self, req: Request<v1::UserProfile>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }

    async fn sign_out(&self, req: Request<()>) -> GrpcResult<()> {
        Ok(Response::new(()))
    }
}

impl Service {
    fn by_nick_name_or_email(db: &Db, id: &str) -> Result<User> {
        let id = id.to_lowercase();
        let id = id.trim();
        if let Ok(it) = UserDao::by_nick_name(db, id) {
            return Ok(it);
        }
        UserDao::by_email(db, id)
    }
    async fn send_email(
        db: &Db,
        jwt: &Jwt,
        queue: &RabbitMq,
        user: &User,
        home: &str,
        lang: &str,
        action: &str,
    ) -> Result<()> {
        // TODO from settings table
        let ttl = Duration::hours(1);
        let (nbf, exp) = Jwt::timestamps(ttl);
        let token = jwt.sum(
            None,
            &Token {
                uid: user.uid.clone(),
                sub: user.real_name.clone(),
                act: action.to_string(),
                nbf,
                exp,
            },
        )?;
        let args = Some(serde_json::json!({
            "token": token,
            "name": user.real_name.clone(),
            "ttl": ttl.num_minutes(),
            "home": home,
        }));
        let task = v1::EmailTask {
            to: user.email.clone(),
            cc: Vec::new(),
            bcc: Vec::new(),
            attachemts: Vec::new(),
            subject: I18n::t(
                db,
                lang,
                format!("auth.users.mailer.{}.subject", action),
                &args,
            ),
            parts: vec![v1::email_task::Part {
                content_type: TEXT_HTML_UTF_8.to_string(),
                body: I18n::t(
                    db,
                    lang,
                    format!("auth.users.mailer.{}.body", action),
                    &args,
                ),
            }],
        };

        let content_type = OCTET_STREAM.as_ref();
        queue
            .publish(v1::EmailTask::QUEUE, content_type, task.encode_to_vec())
            .await?;
        Ok(())
    }
}

impl v1::EmailTask {
    pub const QUEUE: &'static str = "emails";
}
