pub mod requests;
pub mod responses;

use std::fmt;
use std::path::Path;

use axum::{
    body::Body as AxumBody,
    http::{
        Response as AxumResponse,
        header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE},
    },
};
use hyper::StatusCode;
use mime_guess::Mime;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::{
    Client as HttpClient, Response as HttpResponse,
    multipart::{Form as MultipartForm, Part as MultipartPart},
};
use serde::{Deserialize, Serialize};

use super::super::{HttpError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for Config {
    fn default() -> Self {
        Self {
            host: node_default_host(),
            port: node_default_port(),
        }
    }
}

fn node_default_host() -> String {
    "127.0.0.1".to_string()
}

fn node_default_port() -> u16 {
    9333
}

impl Config {
    pub fn open(&self) -> Client {
        let url = self.to_string();
        log::debug!("open SeaweedFS {}", url);
        Client { master_host: url }
    }
}

pub struct Client {
    master_host: String,
}

impl Client {
    // curl http://0.0.0.0:9340/1,0cc0afa344
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
    // curl http://127.0.0.1:9333/cluster/status
    pub async fn cluster_status(&self) -> Result<responses::ClusterStatus> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/cluster/status", self.master_host))
            .send()
            .await?
            .json::<responses::ClusterStatus>()
            .await?;
        log::debug!("{:?}", res);
        Ok(res)
    }
    // curl http://127.0.0.1:9333/dir/lookup?volumeId=1
    pub async fn lookup_volume_by_id(&self, fid: &str) -> Result<responses::LookupVolumeById> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/dir/lookup", self.master_host))
            .query(&requests::LookupVolumeByIdQuery {
                volume_id: Self::volume_id_from_fid(fid)?,
            })
            .send()
            .await?
            .json::<responses::LookupVolumeById>()
            .await?;
        log::debug!("{:?}", res);
        Ok(res)
    }
    // curl http://127.0.0.1:9333/dir/assign
    pub async fn assign(&self) -> Result<responses::Assign> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/dir/assign", self.master_host))
            .send()
            .await?
            .json::<responses::Assign>()
            .await?;
        log::debug!("{:?}", res);
        Ok(res)
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

    // curl http://127.0.0.1:9333/dir/assign
    // curl -F "file=@README.md" http://0.0.0.0:9340/1,108aa6f6ac
    pub async fn upload<P: AsRef<Path>>(
        &self,
        file: P,
        volume: &str,
        fid: &str,
    ) -> Result<responses::UploadFile> {
        let file = file.as_ref();
        let mime = mime_guess::from_path(file)
            .first_or_octet_stream()
            .to_string();
        log::info!(
            "upload file ({}, {}) to ({},{})",
            file.display(),
            mime,
            volume,
            fid
        );

        let part = MultipartPart::file(file).await?;
        let form = MultipartForm::new().part("file", part);

        let client = HttpClient::new();
        let res = client
            .post(Self::url(volume, fid))
            .multipart(form)
            .send()
            .await?
            .json::<responses::UploadFile>()
            .await?;
        Ok(res)
    }

    fn url(volume: &str, fid: &str) -> String {
        format!("http://{volume}/{fid}")
    }
    fn volume_id_from_fid(fid: &str) -> Result<String> {
        let items: Vec<&str> = fid.split(',').collect();
        if items.len() != 2 {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("invalid fid {fid}")),
            )));
        }
        Ok(items[0].to_string())
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
        self.upload(file, bucket, object).await?;

        Ok(())
    }
}
