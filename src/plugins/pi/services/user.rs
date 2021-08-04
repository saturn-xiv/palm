use std::ops::Add;
use std::ops::Deref;
use std::sync::Arc;

use chrono::{Duration as ChronoDuration, Utc};
use hyper::StatusCode;
use prost_types::Duration;
use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::super::super::super::{
    crypto::Aes, jwt::Jwt, orm::sqlite::Connection, request::Token as Auth, GrpcResult, HttpError,
    Result,
};
use super::super::super::nut::{models::user::Token, services::Session};
use super::super::models::settings::Dao as SettingDao;
use super::{user_server::User as UserServer, Context, TokenResponse, UserProfile};

pub struct Service {
    pub ctx: Arc<Context>,
}

#[tonic::async_trait]
impl UserServer for Service {
    async fn set_profile(&self, req: Request<UserProfile>) -> GrpcResult<Response<()>> {
        self.ctx.current_user(&req)?;
        let req = req.into_inner();
        let fm = User {
            name: req.name,
            password: req.password,
        };
        __try_grpc!(fm.validate())?;
        let db = __try_grpc!(self.ctx.db.get())?;
        let db = db.deref();
        let aes = self.ctx.aes.deref();
        __try_grpc!(SettingDao::set(db, aes, User::KEY, &fm, true))?;
        Ok(Response::new(()))
    }
    async fn get_profile(&self, req: Request<()>) -> GrpcResult<Response<UserProfile>> {
        self.ctx.current_user(&req)?;

        let db = __try_grpc!(self.ctx.db.get())?;
        let db = db.deref();
        let aes = self.ctx.aes.deref();
        let user: User = __try_grpc!(SettingDao::get(db, aes, User::KEY))?;

        Ok(Response::new(UserProfile {
            name: user.name.clone(),
            password: user.password,
        }))
    }
    async fn sign_in(&self, req: Request<UserProfile>) -> GrpcResult<Response<TokenResponse>> {
        let req = req.into_inner();

        let db = __try_grpc!(self.ctx.db.get())?;
        let db = db.deref();
        let aes = self.ctx.aes.deref();
        let user: User = __try_grpc!(SettingDao::get(db, aes, User::KEY))?;
        let form = User {
            name: req.name,
            password: req.password,
        };
        if user != form {
            return Err(Status::invalid_argument("bad name and password"));
        }
        let (nbf, exp) = Jwt::timestamps(ChronoDuration::weeks(1));
        let token = __try_grpc!(self.ctx.jwt.sum(
            None,
            &Token {
                uid: user.name.clone(),
                sub: User::KEY.to_string(),
                act: Token::SIGN_IN.to_string(),
                nbf,
                exp,
            },
        ))?;

        warn!("user {} sign in.", user.name);
        Ok(Response::new(TokenResponse { token }))
    }
    async fn token(&self, req: Request<Duration>) -> GrpcResult<Response<TokenResponse>> {
        let user = self.ctx.current_user(&req)?;
        let req = req.into_inner();

        let nbf = Utc::now().naive_utc();
        let exp = nbf.add(ChronoDuration::seconds(req.seconds));

        let token = __try_grpc!(self.ctx.jwt.sum(
            None,
            &Token {
                uid: user.name,
                sub: User::KEY.to_string(),
                act: Token::SIGN_IN.to_string(),
                nbf: nbf.timestamp(),
                exp: exp.timestamp(),
            },
        ))?;

        Ok(Response::new(TokenResponse { token }))
    }
    async fn sign_out(&self, req: Request<()>) -> GrpcResult<Response<()>> {
        let user = self.ctx.current_user(&req)?;
        warn!("user {} sign out.", user.name);
        Ok(Response::new(()))
    }
}

#[derive(Serialize, Deserialize, Validate, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[validate(length(min = 2))]
    pub name: String,
    #[validate(length(min = 6))]
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "admin".to_string(),
            password: "password".to_string(),
        }
    }
}

impl User {
    pub const KEY: &'static str = "administrator";

    pub fn new(db: &Connection, jwt: &Jwt, aes: &Aes, token: &str) -> Result<Self> {
        let token = jwt.parse::<Token>(token)?;
        let token = token.claims;
        if token.act != Token::SIGN_IN {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        let it: User = SettingDao::get(db, aes, User::KEY)?;
        if it.name != token.uid {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        Ok(it)
    }
}

pub trait CurrentUser {
    fn current_user(&self, db: &Connection, jwt: &Jwt, aes: &Aes) -> Result<User>;
}

impl CurrentUser for Auth {
    fn current_user(&self, db: &Connection, jwt: &Jwt, aes: &Aes) -> Result<User> {
        User::new(db, jwt, aes, &self.0)
    }
}

impl CurrentUser for Session {
    fn current_user(&self, db: &Connection, jwt: &Jwt, aes: &Aes) -> Result<User> {
        if let Some(ref token) = self.token {
            return User::new(db, jwt, aes, token);
        }

        Err(Box::new(HttpError(
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            None,
        )))
    }
}

impl Context {
    pub fn current_user<T>(&self, req: &Request<T>) -> GrpcResult<User> {
        let ss = Session::new(req);
        let db = __try_grpc!(self.db.get())?;
        let it = __try_grpc!(
            ss.current_user(&db, &self.jwt, &self.aes),
            Status::unauthenticated
        )?;
        Ok(it)
    }
}
