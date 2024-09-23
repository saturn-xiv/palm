use std::fs::read_to_string;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity, Uri};

use super::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub host: String,
    pub port: u16,
    #[serde(rename = "authority-domain")]
    pub authority_domain: String,
    #[serde(rename = "ca-file")]
    pub ca_file: PathBuf,
    #[serde(rename = "cert-file")]
    pub cert_file: PathBuf,
    #[serde(rename = "key-file")]
    pub key_file: PathBuf,
}

impl Client {
    pub async fn open(&self) -> Result<Channel> {
        let tls = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(read_to_string(&self.ca_file)?))
            .identity(Identity::from_pem(
                read_to_string(&self.cert_file)?,
                read_to_string(&self.key_file)?,
            ))
            .domain_name(&self.authority_domain);

        let channel = Channel::builder(
            Uri::builder()
                .scheme("https")
                .authority(format!("{}:{}", self.host, self.port))
                .build()?,
        )
        .tls_config(tls)?
        .connect()
        .await?;
        Ok(channel)
    }
}
