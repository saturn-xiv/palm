use std::fs::{create_dir_all, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use askama::Template;
use nix::unistd::{Gid, Uid};
use petunia::Result;

#[derive(clap::Parser, PartialEq, Eq, Debug)]
pub struct Command {
    #[clap(short, long)]
    pub domain: String,
}

pub fn launch(domain: &str) -> Result<()> {
    let user = &Uid::current().to_string();
    let group = &Gid::current().to_string();
    let root = Path::new("tmp").join("systemd");
    if !root.exists() {
        create_dir_all(&root)?;
    }
    for it in ["www", "agent"] {
        let file = root.join(format!("{}.{}.service", it, domain));

        let tpl = SystemdConfig {
            user,
            group,
            name: super::NAME,
            domain,
            description: super::DESCRIPTION,
            args: it,
        };
        tpl.write(&file)?;
    }

    log::info!("please copy them into /lib/systemd/system/ folder.");
    Ok(())
}

#[derive(Template)]
#[template(path = "systemd/service.conf", escape = "none")]
struct SystemdConfig<'a> {
    user: &'a str,
    group: &'a str,
    name: &'a str,
    domain: &'a str,
    description: &'a str,
    args: &'a str,
}
impl SystemdConfig<'_> {
    pub fn write<P: AsRef<Path>>(&self, file: P) -> Result<()> {
        let file = file.as_ref();
        log::info!("generate file {}", file.display());
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
