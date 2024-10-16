use std::any::type_name;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration as StdDuration;

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
use daffodil::{controllers as daffodil_controllers, rbac::new as new_enforcer};
use data_encoding::BASE64;
use hyper::StatusCode;
use juniper::EmptySubscription;
use petunia::{
    cache::redis::Config as Redis,
    crypto::Key,
    hostname,
    jwt::openssl::OpenSsl as Jwt,
    opensearch::Config as OpenSearch,
    orm::postgresql::Config as PostgreSql,
    queue::amqp::{Config as RabbitMq, RabbitMq as Queue},
    rbac::v1::WatcherMessage as CasbinWatcherMessage,
    themes::Theme,
    Environment, HttpError, Result,
};
use serde::{Deserialize, Serialize};

use super::super::graphql;
use super::{parse_config_file, NAME};

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Command {
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
    #[clap(short, long, default_value_t = 4)]
    pub threads: usize,
}

impl Command {
    pub async fn launch<P: AsRef<Path>>(&self, config: P) -> Result<()> {
        let config: Config = parse_config_file(config)?;
        let cookie_key = BASE64.decode(config.cookie_key.0.as_bytes())?;
        let is_prod = config.env == Environment::Production;

        let secrets = web::Data::new(config.secrets.clone());
        let db = web::Data::new(config.postgresql.open()?);
        let cache = web::Data::new(config.redis.open()?);
        let jwt = web::Data::new(Jwt::new(config.jwt_key.0.clone()));
        let queue = web::Data::new(config.rabbitmq.open());
        let search = web::Data::new(config.open_search.open()?);
        let enforcer = {
            let db = db.clone();
            let db = db.into_inner();
            let db = db.deref();
            let queue = queue.clone();
            let queue = queue.into_inner();
            let it = new_enforcer(db.clone(), queue).await?;
            web::Data::from(it)
        };
        {
            let name = format!("{}.casbin-watcher", hostname()?);
            let ch = queue.open().await?;
            let queue = type_name::<CasbinWatcherMessage>();
            Queue::queue_declare(&ch, queue, false, false).await?;
            let enf = enforcer.clone();
            tokio::spawn(async move {
                log::info!("start enforcer watcher subscriber({queue})");
                let enf = enf.deref();
                let enf = enf.deref();
                loop {
                    if let Err(e) = Queue::consume(&ch, &name, queue, enf).await {
                        log::error!("casbin watcher subscriber{:?}", e);
                    }
                    tokio::time::sleep(StdDuration::from_secs(5)).await;
                }
            });
        }
        {
            log::debug!("{:?}", search.info());
        }

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
        log::info!("listen on http://{}", addr);
        HttpServer::new(move || {
            App::new()
                .app_data(secrets.clone())
                .app_data(db.clone())
                .app_data(cache.clone())
                .app_data(jwt.clone())
                .app_data(queue.clone())
                .app_data(enforcer.clone())
                .app_data(search.clone())
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
                .configure(petunia::themes::x_corporation::register)
                .configure(graphql::controllers::register)
                .configure(daffodil_controllers::register)
        })
        .workers(self.threads)
        .bind(addr)?
        .run()
        .await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Config {
    theme: Theme,
    env: Environment,
    // openssl rand -base64 128
    #[serde(rename = "cookie-key")]
    cookie_key: Key,
    #[serde(rename = "jwt-key")]
    jwt_key: Key,
    // openssl rand -base64 32
    secrets: Key,
    postgresql: PostgreSql,
    redis: Redis,
    rabbitmq: RabbitMq,
    #[serde(rename = "opensearch")]
    open_search: OpenSearch,
}
