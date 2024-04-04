use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use actix_session::{
    config::{BrowserSession, CookieContentSecurity, SessionLifecycle},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration as CookieDuration, Key as CookieKey, SameSite},
    middleware, web, App, HttpServer,
};
use clap::Parser;
use data_encoding::BASE64;
use log::info;
use palm::{env::Environment, jwt::openssl::Jwt, Result};

use super::super::{
    controllers::{self, logs::Journal},
    NAME,
};
use super::Config;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Server {
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
    #[clap(short, long, default_value_t = 4)]
    pub threads: usize,
    #[clap(short = 'D', long)]
    pub directory: Option<PathBuf>,
}

impl Server {
    pub async fn launch(&self, config: &Config) -> Result<()> {
        let cookie_key = BASE64.decode(config.cookie_key.0.as_bytes())?;
        let is_prod = config.env == Environment::Production;

        let jwt = web::Data::new(Jwt::new(&config.cookie_key.0));
        let journal = web::Data::new(Journal(self.directory.clone()));

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
                .app_data(journal.clone())
                .wrap(middleware::Logger::default())
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
                .configure(controllers::register)
        })
        .workers(self.threads)
        .bind(addr)?
        .run()
        .await?;
        Ok(())
    }
}
