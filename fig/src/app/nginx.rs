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
impl Command {
    pub fn launch(&self) -> Result<()> {
        let root = Path::new("tmp").join("nginx");
        if !root.exists() {
            create_dir_all(&root)?;
        }
        {
            let domain = format!("www.{}", self.domain);
            let tpl = WwwNginxConf {
                domain: &domain,
                name: super::NAME,
                port: self.port,
            };
            let file = root.join(format!("{}.conf", domain));
            tpl.write(&file)?;
        }

        log::info!("please copy it into /etc/nginx/sites-enable/ folder.");
        Ok(())
    }
}
