use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::sync::Arc;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{
    config::{BrowserSession, CookieContentSecurity, SessionLifecycle},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration as CookieDuration, Key as CookieKey, SameSite},
    http::{
        header::{ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE, COOKIE},
        Method,
    },
    middleware, web, App, HttpServer,
};
use chrono::Duration;
use clap::Parser;
use daffodil::controllers as daffodil_controllers;
use data_encoding::BASE64;
use hyper::StatusCode;
use juniper::{EmptyMutation, EmptySubscription};
use log::{debug, info};
use petunia::{
    cache::redis::Config as Redis, crypto::Key, opensearch::Config as OpenSearch,
    orm::postgresql::Config as PostgreSql, queue::amqp::Config as RabbitMq, Environment, HttpError,
    Result,
};
use serde::{Deserialize, Serialize};

use super::super::graphql;
use super::NAME;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Command {
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
    #[clap(short, long, default_value_t = 4)]
    pub threads: usize,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Config {
    env: Environment,
    // openssl rand -base64 128
    #[serde(rename = "cookie-key")]
    cookie_key: Key,
    #[serde(rename = "jwt-key")]
    jwt_key: Key,
    secrets: Key,
    postgresql: PostgreSql,
    redis: Redis,
    rabbitmq: RabbitMq,
    #[serde(rename = "open-search")]
    open_search: OpenSearch,
}
