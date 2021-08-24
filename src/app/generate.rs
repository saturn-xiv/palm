use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use clap::Clap;

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
        let file = file.as_ref();
        match self {
            Self::Config => {
                let buf = toml::to_vec(&env::Config::default())?;
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
                    domain: &cfg.domain,
                    http: &cfg.http,
                    grpc: &cfg.grpc,
                    ssl: it.ssl,
                };
                it.store("tmp")?;
            }
            Self::Systemd => {
                let cfg: env::Config = from_toml(file)?;
                for action in &["web", "rpc", "worker"] {
                    let it = env::SystemdService {
                        domain: &cfg.domain,
                        config: &file.display().to_string(),
                        name: NAME,
                        description: DESCRIPTION,
                        action,
                    };
                    it.store("tmp")?;
                }
            }
        };
        Ok(())
    }
}

#[derive(Clap)]
pub struct Nginx {
    #[clap(short, long)]
    pub ssl: bool,
}
