use juniper::{GraphQLInputObject, GraphQLObject};
use palm::{crypto::tink::HMac, queue::rabbitmq::amqp::Connection as RabbitMq, Result};

use super::super::orm::postgresql::Connection as Db;

#[derive(GraphQLInputObject)]
#[graphql(name = "UserSignInRequest")]
pub struct SignInRequest {
    pub user: String,
    pub password: String,
    pub ttl: Option<i32>,
}

impl SignInRequest {
    pub fn execute(&self, _db: &mut Db, _mac: &HMac) -> Result<SignInResponse> {
        // TODO
        todo!()
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "UserSignInResponse")]
pub struct SignInResponse {
    pub token: String,
    pub real_name: String,
    pub roles: Vec<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(name = "UserSignUpRequest")]
pub struct SignUpRequest {
    pub nickname: String,
    pub real_name: String,
    pub email: String,
    pub password: String,
    pub lang: String,
    pub timezone: String,
}

impl SignUpRequest {
    pub fn execute(&self, _db: &mut Db, _mac: &HMac, _rabbitmq: &RabbitMq) -> Result<()> {
        // TODO
        todo!()
    }
}
