pub mod os;
pub mod user;

use std::sync::Arc;

use super::super::super::{crypto::Aes, jwt::Jwt, orm::sqlite::Pool as DbPool};

pub struct Context {
    pub db: DbPool,
    pub jwt: Arc<Jwt>,
    pub aes: Arc<Aes>,
}

tonic::include_proto!("palm.pi.v1");
