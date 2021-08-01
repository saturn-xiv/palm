use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use clap::Clap;
use nix::unistd::{Gid, Uid};

use super::super::{env, parser::from_toml, Result, DESCRIPTION, NAME};

#[derive(Clap)]
pub enum Generate {
    #[clap(about = "Generate config.toml")]
    Config,
    #[clap(about = "Generate nginx.conf")]
    Nginx(Nginx),
    #[clap(about = "Generate systemd.conf")]
    Systemd,
}

impl Generate {
    pub fn launch<P: AsRef<Path>>(&self, file: P) -> Result<()> {
        match self {
            Self::Config => {
                let buf = toml::to_vec(&env::Config::default())?;
                let file = file.as_ref();
                info!("generate file {}", file.display());
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(file)?;
                file.write_all(&buf)?;
            }
            Self::Nginx(it) => {
                let cfg: env::Config = from_toml(file)?;
                let it = env::NginxConf {
                    name: NAME,
                    domain: &it.domain,
                    ssl: it.ssl,
                    port: cfg.port,
                };
                it.store("tmp")?;
            }
            Self::Systemd => {
                let cur = current_dir()?;
                let it = env::SystemdService {
                    user: &Uid::current().to_string(),
                    group: &Gid::current().to_string(),
                    name: NAME,
                    description: DESCRIPTION,
                    root: &format!("{}", cur.display()),
                };
                it.store("tmp")?;
            }
        };
        Ok(())
    }
}

#[derive(Clap)]
pub struct Nginx {
    #[clap(short, long)]
    pub domain: String,
    #[clap(short, long)]
    pub ssl: bool,
}
