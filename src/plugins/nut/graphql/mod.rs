pub mod policy;
pub mod user;

use std::sync::Arc;

use chrono::{NaiveDateTime, Utc};
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};

use super::super::super::{
    cache::redis::Pool as CachePool,
    crypto::{Aes, Hmac},
    jwt::Jwt,
    orm::postgresql::{Connection as Db, Pool as DatabasePool, PooledConnection as PooledDb},
    queue::amqp::RabbitMq,
    HttpError, Result,
};
use super::models::user::{Dao as UserDao, Item as User, Token as UserToken};

pub const ROOT: &str = "/graphql";

#[derive(GraphQLObject)]
#[graphql(name = "Ok")]
pub struct Ok_ {
    pub created_at: NaiveDateTime,
}
impl Default for Ok_ {
    fn default() -> Self {
        Self {
            created_at: Utc::now().naive_local(),
        }
    }
}

#[derive(Default)]
pub struct Session {
    pub client_ip: String,
    pub token: Option<String>,
    pub locale: String,
}

impl Session {
    pub fn current_user(&self, db: &Db, jwt: &Jwt) -> Result<User> {
        if let Some(ref token) = self.token {
            let token = jwt.parse::<UserToken>(token)?.claims;
            if token.act == UserToken::SIGN_IN {
                let user = UserDao::by_uid(db, &token.uid)?;
                user.available()?;
                return Ok(user);
            }
        }
        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }
}

#[derive(GraphQLObject)]
pub struct Pagination {
    pub index: i32,
    pub total: i32,
    pub size: i32,
}

impl Pagination {
    pub fn new(total: i64, page: &Page) -> Self {
        let (index, size) = page.verify(total);
        Self {
            index: index as i32,
            size: size as i32,
            total: total as i32,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct Page {
    pub index: i32,
    pub size: i32,
}

impl Page {
    pub const MIN_SIZE: i64 = 1 << 4;
    pub const MAX_SIZE: i64 = 1 << 10;

    pub fn verify(&self, total: i64) -> (i64, i64) {
        let size = if (self.size as i64) <= Self::MIN_SIZE {
            Self::MIN_SIZE
        } else if (self.size as i64) >= Self::MAX_SIZE {
            Self::MAX_SIZE
        } else {
            self.size as i64
        };
        let index = if self.index <= 1 {
            1
        } else if ((self.index as i64) * size) <= total {
            self.index as i64
        } else if total % size == 0 {
            total / size
        } else {
            (total / size) + 1
        };
        (index, size)
    }

    pub fn pagination(&self, total: i64) -> (i64, i64) {
        let (index, size) = self.verify(total);
        ((index - 1) * size, size)
    }
}

pub struct Context {
    pub aes: Arc<Aes>,
    pub jwt: Arc<Jwt>,
    pub hmac: Arc<Hmac>,
    pub db: DatabasePool,
    pub cache: CachePool,
    pub queue: Arc<RabbitMq>,
    pub session: Session,
}
impl juniper::Context for Context {}

impl Context {
    pub fn db(&self) -> Result<PooledDb> {
        let it = self.db.get()?;
        Ok(it)
    }
}
