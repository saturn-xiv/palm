use std::path::PathBuf;

pub struct GRpcClient {
    pub host: String,
    pub port: u16,
    pub ca_file: PathBuf,
    pub cert_file: PathBuf,
    pub key_file: PathBuf,
}
