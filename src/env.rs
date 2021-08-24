use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use askama::Template;
use serde::{Deserialize, Serialize};

use super::{
    aws::s3::Config as S3, cache::redis::Config as Redis, crypto::Key,
    orm::postgresql::Config as PostgreSql, queue::amqp::Config as RabbitMq, Result,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub port: u16,
}

impl Default for Http {
    fn default() -> Self {
        Self { port: 8080 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Grpc {
    pub port: u16,
}

impl Default for Grpc {
    fn default() -> Self {
        Self { port: 8088 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub domain: String,
    pub secrets: String,
    pub http: Http,
    pub grpc: Grpc,
    pub postgresql: PostgreSql,
    pub redis: Redis,
    pub rabbitmq: RabbitMq,
    pub s3: S3,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            domain: "www.change-me.org".to_string(),
            secrets: Key::default().0,
            postgresql: PostgreSql::default(),
            redis: Redis::default(),
            rabbitmq: RabbitMq::default(),
            grpc: Grpc::default(),
            http: Http::default(),
            s3: S3::default(),
        }
    }
}

#[derive(Template)]
#[template(path = "nginx.conf", escape = "none")]
pub struct NginxConf<'a> {
    pub domain: &'a str,
    pub name: &'a str,
    pub ssl: bool,
    pub http: &'a Http,
    pub grpc: &'a Grpc,
}

impl<'a> NginxConf<'a> {
    pub fn store<P: AsRef<Path>>(&self, root: P) -> Result<()> {
        let root = root.as_ref();
        let body = self.render()?;
        let file = root.join(format!("{}.conf", self.name));
        info!("generate file {}", file.display());
        let mut file = File::create(&file)?;
        file.write_all(body.as_bytes())?;
        info!("please copy it into /etc/nginx/sites-enabled folder.");
        Ok(())
    }
}

#[derive(Template)]
#[template(path = "systemd-service.conf", escape = "none")]
pub struct SystemdService<'a> {
    pub domain: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub action: &'a str,
    pub config: &'a str,
}

impl<'a> SystemdService<'a> {
    pub fn store<P: AsRef<Path>>(&self, root: P) -> Result<()> {
        let root = root.as_ref();
        let body = self.render()?;
        let file = root.join(format!("api.{}.service", self.name));
        info!("generate file {}", file.display());
        let mut file = File::create(&file)?;
        file.write_all(body.as_bytes())?;
        info!("please copy it into /lib/systemd/system/ folder.");
        Ok(())
    }
}
