pub mod user;

use juniper::{GraphQLInputObject, GraphQLObject};

pub const QUEUE_SMS_BY_TWILIO: &str = "sms-send.twilio";
pub const QUEUE_TEX: &str = "tex";
pub const QUEUE_EMAIL_SEND: &str = "email-send";
pub const QUEUE_CUPS: &str = "cups";

pub struct Session {
    pub client_ip: String,
}

pub struct Plugin;

#[derive(Debug, GraphQLObject)]
#[graphql(name = "Pagination")]
pub struct Pagination {
    pub page: i32,
    pub size: i32,
    pub total: i32,
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(name = "Page")]
pub struct Page {
    pub index: i32,
    pub size: i32,
}
