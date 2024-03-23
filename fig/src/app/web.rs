use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Deref;
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
use camelia::{controllers as camelia_controllers, orm::postgresql::Config as PostgreSql};
use chrono::Duration;
use clap::Parser;
use daffodil::controllers as daffodil_controllers;
use data_encoding::BASE64;
use hyper::StatusCode;
use juniper::EmptySubscription;
use log::{debug, info};
use palm::{
    cache::redis::Config as Redis,
    crypto::{aes::Aes, hmac::Hmac, Key},
    env::Environment,
    jwt::openssl::Jwt,
    minio::Config as Minio,
    parser::from_toml,
    queue::rabbitmq::{amqp::Amqp, Config as RabbitMq},
    search::Config as OpenSearch,
    HttpError, Result,
};
use serde::{Deserialize, Serialize};

use super::super::{graphql, NAME};

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Server {
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
    #[clap(short, long, default_value_t = 4)]
    pub threads: usize,
}

impl Server {
    pub async fn launch<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: Config = from_toml(config_file)?;

        let cookie_key = BASE64.decode(config.cookie_key.0.as_bytes())?;
        let is_prod = config.env == Environment::Production;

        let pg_pool = config.postgresql.open()?;

        let pgsql = web::Data::new(pg_pool);
        let redis = web::Data::new(config.redis.open()?);
        let rabbitmq = web::Data::new(config.rabbitmq.open().await?);
        let minio = web::Data::new(config.minio.open()?);
        {
            let items = minio.connection.list_buckets().await?;
            debug!("found buckets: {:?}", items);
        }
        let opensearch = web::Data::new(config.opensearch.open()?);
        let jwt = web::Data::new(Jwt::new(&config.cookie_key.0));
        let hmac = web::Data::new(Hmac::new(&config.secret_key.0)?);
        let aes = web::Data::new(Aes::new(&config.secret_key.0)?);
        let enforcer = {
            let rabbitmq = rabbitmq.deref();
            let rabbitmq = rabbitmq.clone();
            web::Data::from(config.postgresql.casbin_enforcer(rabbitmq).await?)
        };

        let schema = Arc::new(graphql::Schema::new(
            graphql::query::Query {},
            graphql::mutation::Mutation {},
            EmptySubscription::new(),
        ));
        let cookie_max_age = Duration::try_hours(1)
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("bad cookie max age".to_string()),
            )))?
            .num_seconds() as usize;

        let addr = SocketAddr::new(
            IpAddr::V4(if is_prod {
                Ipv4Addr::new(127, 0, 0, 1)
            } else {
                Ipv4Addr::new(0, 0, 0, 0)
            }),
            self.port,
        );
        info!("listen on http://{}", addr);
        HttpServer::new(move || {
            App::new()
                .app_data(jwt.clone())
                .app_data(aes.clone())
                .app_data(hmac.clone())
                .app_data(pgsql.clone())
                .app_data(redis.clone())
                .app_data(rabbitmq.clone())
                .app_data(minio.clone())
                .app_data(opensearch.clone())
                .app_data(enforcer.clone())
                .app_data(web::Data::from(schema.clone()))
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allowed_methods(vec![Method::POST, Method::GET])
                        .allowed_headers(vec![
                            AUTHORIZATION,
                            ACCEPT,
                            CONTENT_TYPE,
                            ACCEPT_LANGUAGE,
                            COOKIE,
                        ])
                        .allowed_header("X-Requested-With")
                        .supports_credentials()
                        .allow_any_origin()
                        .max_age(cookie_max_age),
                )
                .wrap(middleware::Logger::default())
                .wrap(IdentityMiddleware::default())
                .wrap(
                    SessionMiddleware::builder(
                        CookieSessionStore::default(),
                        CookieKey::from(&cookie_key),
                    )
                    .cookie_name(format!("{}.ss", NAME))
                    .cookie_same_site(SameSite::Strict)
                    .cookie_http_only(true)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_path("/".to_string())
                    .session_lifecycle(SessionLifecycle::BrowserSession(
                        BrowserSession::default().state_ttl(CookieDuration::hours(1)),
                    ))
                    .cookie_secure(is_prod)
                    .build(),
                )
                .configure(graphql::controllers::register)
                .configure(camelia_controllers::register)
                .configure(daffodil_controllers::register)
        })
        .workers(self.threads)
        .bind(addr)?
        .run()
        .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rpc {
    pub host: String,
    pub port: u16,
}

impl Default for Rpc {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    // openssl rand -base64 128
    #[serde(rename = "cookie-key")]
    pub cookie_key: Key,
    // openssl rand -base64 32
    #[serde(rename = "secret-key")]
    pub secret_key: Key,
    // pub musa: Rpc,
    // pub orchid: Rpc,
    pub postgresql: PostgreSql,
    pub redis: Redis,
    pub rabbitmq: RabbitMq,
    pub opensearch: OpenSearch,
    pub minio: Minio,
}
