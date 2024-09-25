use std::net::{IpAddr, Ipv4Addr, SocketAddr};
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
use data_encoding::BASE64;
use hyper::StatusCode;
use juniper::{EmptyMutation, EmptySubscription};
use petunia::{
    crypto::Key, jwt::openssl::OpenSsl as Jwt, opensearch::Config as OpenSearch, Environment,
    HttpError, Result,
};
use serde::{Deserialize, Serialize};

use super::super::graphql;
use super::NAME;

#[derive(clap::Parser, PartialEq, Eq, Debug)]
pub struct Command {
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    #[clap(short, long, default_value = "32")]
    pub threads: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    pub secrets: Key,
    #[serde(rename = "systemd-journal-index-name")]
    pub systemd_journal_index_name: String,
    #[serde(rename = "snmp-index-name")]
    pub snmp_index_name: String,
    #[serde(rename = "open-search")]
    pub open_search: OpenSearch,
}

pub async fn launch(config: &Config, port: u16, threads: usize) -> Result<()> {
    let cookie_key = BASE64.decode(config.secrets.0.as_bytes())?;
    let is_prod = config.env == Environment::Production;

    let jwt = web::Data::new(Jwt::new(config.secrets.0.clone()));

    let search = web::Data::new(config.open_search.open()?);
    {
        log::debug!("{:?}", search.info());
    }

    let schema = Arc::new(graphql::Schema::new(
        graphql::query::Query {},
        EmptyMutation::new(),
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
        port,
    );
    log::info!("listen on http://{}", addr);
    HttpServer::new(move || {
        App::new()
            .app_data(jwt.clone())
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
            .configure(graphql::controllers::register)
    })
    .workers(threads)
    .bind(addr)?
    .run()
    .await?;
    Ok(())
}
