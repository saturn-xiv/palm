pub mod responses;

use std::ops::DerefMut;
use std::path::Path;

use diesel::Connection as DieselConnection;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use tokio::fs::File;

use super::super::{
    Error, HttpError, Result, models::attachment::Dao as AttachmentDao,
    orm::postgresql::Pool as DbPool,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "node_default_host")]
    pub host: String,
    #[serde(default = "node_default_port")]
    pub port: u16,
}

fn node_default_host() -> String {
    "127.0.0.1".to_string()
}

fn node_default_port() -> u16 {
    9333
}

impl Config {
    pub async fn open(&self, db: DbPool) -> Result<Client> {
        let it = Client {
            master_host: format!("http://{}:{}", self.host, self.port),
            db,
        };
        it.ping().await?;
        Ok(it)
    }
}

pub struct Client {
    db: DbPool,
    master_host: String,
}

impl Client {
    pub async fn ping(&self) -> Result<()> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/cluster/status", self.master_host))
            .send()
            .await?;
        let status = res.status();
        let res_body = res.text().await?;
        log::debug!("SeaweedFS health: {status} {res_body}");
        if !status.is_success() {
            return Err(Box::new(HttpError(status, Some(res_body))));
        }
        Ok(())
    }
    pub async fn assign(&self) -> Result<(String, String)> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/dir/assign", self.master_host))
            .send()
            .await?
            .json::<responses::AssignResponse>()
            .await?;
        Ok((res.fid, res.url))
    }
    pub async fn upload<P: AsRef<Path>>(&self, file: P, url: &str, fid: &str) -> Result<()> {
        let file = file.as_ref();
        log::info!("upload file {} to volumn {}/{}", file.display(), url, fid);

        let client = HttpClient::new();
        let res = client
            .post(format!("http://{}/{}", url, fid))
            .body(File::open(file).await?)
            .send()
            .await?;

        let status = res.status();
        let res_body = res.text().await?;
        log::debug!("{status} {res_body}");
        if !status.is_success() {
            return Err(Box::new(HttpError(status, Some(res_body))));
        }
        Ok(())
    }
}

impl super::Provider for Client {
    async fn upload<P: AsRef<Path>>(&self, file: P, bucket: &str, object: &str) -> Result<()> {
        self.upload(file, bucket, object).await?;
        {
            let mut db = self.db.get()?;
            let db = db.deref_mut();

            db.transaction::<_, Error, _>(|tx| {
                let it = AttachmentDao::by_bucket_and_object(tx, bucket, object)?;
                AttachmentDao::set_uploaded_at(tx, it.id)?;
                Ok(())
            })?;
        }
        Ok(())
    }
}
