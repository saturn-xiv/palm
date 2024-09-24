use std::fs::create_dir_all;
use std::path::Path;

use petunia::{network::nginx::Www as WwwNginxConf, Result};

#[derive(clap::Parser, PartialEq, Eq, Debug)]
pub struct Command {
    #[clap(short, long)]
    pub domain: String,
    #[clap(short, long)]
    pub port: u16,
}

pub fn launch(domain: &str, port: u16) -> Result<()> {
    let root = Path::new("tmp").join("nginx");
    if !root.exists() {
        create_dir_all(&root)?;
    }
    {
        let domain = format!("www.{}", domain);
        let tpl = WwwNginxConf {
            domain: &domain,
            name: super::NAME,
            port,
        };
        let file = root.join(format!("{}.conf", domain));
        tpl.write(&file)?;
    }

    log::info!("please copy it into /etc/nginx/sites-enable/ folder.");
    Ok(())
}
