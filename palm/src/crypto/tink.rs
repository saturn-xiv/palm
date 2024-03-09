use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use hyper::StatusCode;
use tink_aead::aes256_gcm_siv_key_template;
use tink_core::{
    keyset::{BinaryReader, BinaryWriter, Handle},
    Aead, Mac, TinkError,
};
use tink_mac::hmac_sha512_tag512_key_template;
use tink_proto::KeyTemplate;

use super::super::{Error, HttpError, Result};

pub struct Aes {
    key: Box<dyn Aead>,
}

impl Aes {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self> {
        tink_aead::init();
        let key = load(file, &aes256_gcm_siv_key_template())?;
        Ok(Self {
            key: tink_aead::new(&key).map_err(map_tink_err)?,
        })
    }
}

impl super::Secret for Aes {
    fn encrypt(&self, plain_text: &[u8], additional_data: &[u8]) -> Result<Vec<u8>> {
        let it = self
            .key
            .encrypt(plain_text, additional_data)
            .map_err(map_tink_err)?;
        Ok(it)
    }
    fn decrypt(&self, cipher_text: &[u8], additional_data: &[u8]) -> Result<Vec<u8>> {
        let it = self
            .key
            .decrypt(cipher_text, additional_data)
            .map_err(map_tink_err)?;
        Ok(it)
    }
}
pub struct HMac {
    key: Box<dyn Mac>,
}

impl HMac {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self> {
        tink_mac::init();
        let key = load(file, &hmac_sha512_tag512_key_template())?;
        Ok(Self {
            key: tink_mac::new(&key).map_err(map_tink_err)?,
        })
    }
}

impl super::Password for HMac {
    fn compute(&self, plain_text: &[u8]) -> Result<Vec<u8>> {
        let it = self.key.compute_mac(plain_text).map_err(map_tink_err)?;
        Ok(it)
    }
    fn verify(&self, cipher_text: &[u8], plain_text: &[u8]) -> bool {
        self.key.verify_mac(cipher_text, plain_text).is_ok()
    }
}

fn load<P: AsRef<Path>>(file: P, tpl: &KeyTemplate) -> Result<Handle> {
    tink_aead::init();
    {
        let file = file.as_ref();
        if !file.exists() {
            let key = Handle::new(tpl).map_err(map_tink_err)?;
            {
                warn!("generate keyset file {}", file.display());
                let file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(file)?;
                let mut fbw = BinaryWriter::new(file);
                key.write_with_no_secrets(&mut fbw).map_err(map_tink_err)?;
            }
        }
    }
    let key = {
        let file = File::open(file)?;
        let mut fbr = BinaryReader::new(file);
        Handle::read_with_no_secrets(&mut fbr).map_err(map_tink_err)?
    };
    Ok(key)
}

pub fn map_tink_err(e: TinkError) -> Error {
    Box::new(HttpError(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(e.to_string()),
    ))
}
