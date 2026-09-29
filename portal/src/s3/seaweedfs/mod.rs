pub mod responses;

use std::fmt;
use std::fs;
use std::ops::DerefMut;
use std::path::Path;

use axum::{
    body::Body as AxumBody,
    http::{
        Response as AxumResponse,
        header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE},
    },
};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use mime_guess::Mime;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::{
    Client as HttpClient, Response as HttpResponse,
    multipart::{Form as MultipartForm, Part as MultipartPart},
};
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
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "http://{}:{}", self.host, self.port)
    }
}

fn node_default_host() -> String {
    "127.0.0.1".to_string()
}

fn node_default_port() -> u16 {
    9333
}

impl Config {
    pub async fn open(&self, db: DbPool) -> Result<Client> {
        let url = self.to_string();
        log::debug!("open SeaweedFS {}", url);
        let it = Client {
            master_host: url,
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
    pub async fn show(
        &self,
        volume: &str,
        fid: &str,
        title: &str,
        download: bool,
    ) -> Result<AxumResponse<AxumBody>> {
        log::debug!("get file ({title}, {volume}, {fid})");
        let url = Self::url(volume, fid);
        let client = reqwest::Client::new();
        let res = client.get(&url).send().await?;

        if !res.status().is_success() {
            return Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)));
        }

        // Extract headers to pass along to the client
        let content_type = res
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|x| x.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let content_length = res
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|x| x.to_str().ok())
            .map(|x| x.to_string());

        let stream = res.bytes_stream();

        let body = AxumBody::from_stream(stream);

        let mut builder = AxumResponse::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, content_type);

        if let Some(len) = content_length {
            builder = builder.header(CONTENT_LENGTH, len);
        }
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Disposition
        if download {
            let title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
            log::debug!("download {title}");
            builder = builder.header(
                CONTENT_DISPOSITION,
                format!("attachment; filename*=UTF-8''{}", title),
            );
        } else {
            builder = builder.header(CONTENT_DISPOSITION, "inline");
        }

        Ok(builder.body(body)?)
    }
    pub async fn ping(&self) -> Result<()> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/cluster/status", self.master_host))
            .send()
            .await?;
        Self::check(res).await?;
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
        log::debug!("{:?}", res);
        Ok((res.url, res.fid))
    }

    pub async fn write(
        &self,
        name: &str,
        content_type: &Mime,
        (chunk, index): (Vec<u8>, usize),
        volume: &str,
        fid: &str,
    ) -> Result<()> {
        let content_type = content_type.to_string();
        let url = {
            let mut it = Self::url(volume, fid);
            if index != 0 {
                it = format!("{}_{}", it, index);
            }
            it
        };
        log::debug!("uploading ({name},{content_type}) => ({volume},{fid},{index})");
        let form = MultipartForm::new().part(
            "file",
            MultipartPart::bytes(chunk)
                .file_name(name.to_string())
                .mime_str(&content_type)?,
        );
        let client = HttpClient::new();
        let res = client.post(url).multipart(form).send().await?;
        Self::check(res).await?;
        Ok(())
    }

    pub async fn upload<P: AsRef<Path>>(&self, file: P, volume: &str, fid: &str) -> Result<()> {
        let file = file.as_ref();
        log::info!("upload file {} to ({},{})", file.display(), volume, fid);

        let client = HttpClient::new();
        let res = client
            .post(Self::url(volume, fid))
            .body(File::open(file).await?)
            .send()
            .await?;

        Self::check(res).await?;
        Ok(())
    }

    fn url(volume: &str, fid: &str) -> String {
        format!("http://{volume}/{fid}")
    }

    async fn check(res: HttpResponse) -> Result<String> {
        let status = res.status();
        let body = res.text().await?;
        log::debug!("{status} {body}");
        if !status.is_success() {
            return Err(Box::new(HttpError(status, Some(body))));
        }
        Ok(body)
    }
}

impl super::Provider for Client {
    async fn upload<P: AsRef<Path>>(&self, file: P, bucket: &str, object: &str) -> Result<()> {
        let file = file.as_ref();
        let size = {
            let md = fs::metadata(file)?;
            md.len()
        };
        self.upload(file, bucket, object).await?;
        {
            let mut db = self.db.get()?;
            let db = db.deref_mut();

            db.transaction::<_, Error, _>(|tx| {
                let it = AttachmentDao::by_bucket_and_object(tx, bucket, object)?;
                AttachmentDao::set_uploaded_at(tx, it.id, size as usize)?;
                Ok(())
            })?;
        }
        Ok(())
    }
}
