use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use askama::Template;
use palm::Result;

#[derive(Template)]
#[template(path = "minio/service.conf", escape = "none")]
pub struct SystemdConfig<'a> {
    pub domain: &'a str,
    pub port: u16,
    pub console_port: u16,
    pub user: &'a str,
    pub password: &'a str,
}

impl SystemdConfig<'_> {
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

#[derive(Template)]
#[template(path = "minio/nginx.conf", escape = "none")]
pub struct NginxConfig<'a> {
    pub domain: &'a str,
    pub nodes: &'a [Node<'a>],
}

pub struct Node<'a> {
    pub host: &'a str,
    pub port: u16,
    pub console_port: u16,
}

impl NginxConfig<'_> {
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
