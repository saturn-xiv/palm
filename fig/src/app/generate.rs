use std::fs::{create_dir_all, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use askama::Template;
use clap::Parser;
use hibiscus::{
    minio::{
        NginxConfig as MinioNginxConfig, Node as MinioNode, SystemdConfig as MinioSystemdConfig,
    },
    network::nginx::Www as WwwNginxConf,
};
use log::info;
use nix::unistd::{Gid, Uid};
use palm::{random::string as random_string, Result, DESCRIPTION, NAME};

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Nginx {
    #[clap(short, long)]
    pub domain: String,
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
    #[clap(long, default_value = "127.0.0.1")]
    pub minio_host: String,
    #[clap(long, default_value_t = 9000)]
    pub minio_port: u16,
    #[clap(long, default_value_t = 9001)]
    pub minio_console_port: u16,
}

impl Nginx {
    pub fn launch(&self) -> Result<()> {
        let root = Path::new("tmp").join("nginx");
        if !root.exists() {
            create_dir_all(&root)?;
        }
        {
            let domain = format!("www.{}", self.domain);
            let tpl = WwwNginxConf {
                domain: &domain,
                name: NAME,
                port: self.port,
            };
            let file = root.join(format!("{}.conf", domain));
            info!("write to {}", file.display());
            tpl.write(&file)?;
        }
        {
            let domain = format!("s3.{}", self.domain);
            let node = MinioNode {
                host: &self.minio_host,
                port: self.minio_port,
                console_port: self.minio_console_port,
            };
            let tpl = MinioNginxConfig {
                domain: &domain,
                nodes: &[node],
            };
            let file = root.join(format!("{}.conf", domain));
            info!("write to {}", file.display());
            tpl.write(&file)?;
        }

        info!("please copy it into /etc/nginx/sites-enable/ folder.");
        Ok(())
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Systemd {
    #[clap(short, long)]
    pub domain: String,
    #[clap(long, default_value_t = 9000)]
    pub port: u16,
    #[clap(long, default_value_t = 9001)]
    pub console_port: u16,
}

impl Systemd {
    pub fn launch(&self) -> Result<()> {
        let user = &Uid::current().to_string();
        let group = &Gid::current().to_string();
        let root = Path::new("tmp").join("systemd");
        if !root.exists() {
            create_dir_all(&root)?;
        }
        for it in ["www", "rpc", "worker"] {
            let file = root.join(format!("{}.{}.service", it, self.domain));

            let tpl = PalmSystemdConfig {
                user,
                group,
                name: NAME,
                domain: &self.domain,
                description: DESCRIPTION,
                args: it,
            };
            tpl.write(&file)?;
        }
        {
            let file = root.join(format!("s3.{}.service", self.domain));

            let tpl = MinioSystemdConfig {
                domain: &self.domain,
                user: "root",
                port: self.port,
                console_port: self.console_port,
                password: &random_string(16),
            };
            tpl.write(&file)?;
        }

        info!("please copy them into /lib/systemd/system/ folder.");
        Ok(())
    }
}

#[derive(Template)]
#[template(path = "systemd/palm.conf", escape = "none")]
struct PalmSystemdConfig<'a> {
    user: &'a str,
    group: &'a str,
    name: &'a str,
    domain: &'a str,
    description: &'a str,
    args: &'a str,
}
impl PalmSystemdConfig<'_> {
    pub fn write<P: AsRef<Path>>(&self, file: P) -> Result<()> {
        let file = file.as_ref();
        info!("generate file {}", file.display());
        let tpl = self.render()?;
        let mut fd = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(file)?;
        fd.write_all(tpl.as_bytes())?;
        Ok(())
    }
}
