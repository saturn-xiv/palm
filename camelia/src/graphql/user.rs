use juniper::{GraphQLInputObject, GraphQLObject};
use palm::{crypto::hmac::Hmac, queue::rabbitmq::amqp::Connection as RabbitMq, Result};
use validator::Validate;

use super::super::orm::postgresql::Connection as Db;

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "UserSignInRequest")]
pub struct SignInRequest {
    #[validate(length(min = 1, max = 31))]
    pub user: String,
    #[validate(length(min = 1, max = 31))]
    pub password: String,
    #[validate(range(min = 60, max = 604800))]
    pub ttl: Option<i32>,
}

impl SignInRequest {
    pub fn execute(&self, _db: &mut Db, _mac: &Hmac) -> Result<SignInResponse> {
        self.validate()?;
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

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "UserSignUpRequest")]
pub struct SignUpRequest {
    #[validate(email, length(min = 2, max = 31))]
    pub nickname: String,
    #[validate(email, length(min = 2, max = 31))]
    pub real_name: String,
    #[validate(email, length(min = 6, max = 127))]
    pub email: String,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
    #[validate(length(min = 1, max = 17))]
    pub lang: String,
    #[validate(length(min = 3, max = 31))]
    pub timezone: String,
}

impl SignUpRequest {
    pub fn execute(&self, _db: &mut Db, _mac: &Hmac, _rabbitmq: &RabbitMq) -> Result<()> {
        self.validate()?;
        // TODO
        todo!()
    }
}
