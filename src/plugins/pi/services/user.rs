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
    auth::{models::user::Token, services::Session},
    crypto::Aes,
    jwt::Jwt,
    orm::sqlite::{Connection, Pool as DbPool},
    request::Token as Auth,
    GrpcResult, HttpError, Result,
};
use super::super::{
    models::settings::Dao as SettingDao,
    v1::{user_server::User as UserServer, TokenResponse, UserProfile},
};

pub struct Service {
    pub db: DbPool,
    pub jwt: Arc<Jwt>,
    pub aes: Arc<Aes>,
}

#[tonic::async_trait]
impl UserServer for Service {
    async fn set_profile(&self, req: Request<UserProfile>) -> GrpcResult<()> {
        current_pi_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();

        let req = req.into_inner();
        let fm = User {
            name: req.name,
            password: req.password,
        };
        try_grpc!(fm.validate())?;
        let aes = self.aes.deref();
        try_grpc!(SettingDao::set(db, aes, User::KEY, &fm, true))?;
        Ok(Response::new(()))
    }
    async fn get_profile(&self, req: Request<()>) -> GrpcResult<UserProfile> {
        current_pi_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        let aes = self.aes.deref();
        let user: User = try_grpc!(SettingDao::get(db, aes, User::KEY))?;

        Ok(Response::new(UserProfile {
            name: user.name,
            password: String::new(),
        }))
    }
    async fn sign_in(&self, req: Request<UserProfile>) -> GrpcResult<TokenResponse> {
        let req = req.into_inner();

        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        let aes = self.aes.deref();
        let user: User = try_grpc!(SettingDao::get(db, aes, User::KEY))?;
        let form = User {
            name: req.name,
            password: req.password,
        };
        if user != form {
            return Err(Status::invalid_argument("bad name and password"));
        }
        let (nbf, exp) = Jwt::timestamps(ChronoDuration::weeks(1));
        let token = try_grpc!(self.jwt.sum(
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
    async fn token(&self, req: Request<Duration>) -> GrpcResult<TokenResponse> {
        let user = current_pi_user!(self, &req);
        let req = req.into_inner();

        let nbf = Utc::now().naive_utc();
        let exp = nbf.add(ChronoDuration::seconds(req.seconds));

        let token = try_grpc!(self.jwt.sum(
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
    async fn sign_out(&self, req: Request<()>) -> GrpcResult<()> {
        let user = current_pi_user!(self, &req);
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
