use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use askama::Template;

use super::Result;

#[derive(Template)]
#[template(path = "nginx.conf", escape = "none")]
pub struct NginxConf<'a> {
    pub domain: &'a str,
    pub name: &'a str,
    pub ssl: bool,
    pub port: u16,
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
    pub user: &'a str,
    pub group: &'a str,
    pub name: &'a str,
    pub root: &'a str,
    pub description: &'a str,
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
