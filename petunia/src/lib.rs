pub mod cache;
pub mod crypto;
pub mod google;
pub mod grpc;
pub mod handlers;
pub mod jwt;
pub mod network;
pub mod opensearch;
pub mod orm;
pub mod parser;
pub mod queue;
pub mod result;
pub mod session;
pub mod wechat;

pub mod balsam {
    pub mod v1 {
        tonic::include_proto!("palm.balsam.v1");
    }
}
pub mod daisy {
    pub mod v1 {
        tonic::include_proto!("palm.daisy.v1");
    }
}
pub mod lily {
    pub mod v1 {
        tonic::include_proto!("palm.lily.v1");
    }
}
pub mod morus {
    pub mod v1 {
        tonic::include_proto!("palm.morus.v1");
    }
}
pub mod rbac {
    pub mod v1 {
        tonic::include_proto!("palm.rbac.v1");
    }
}
pub mod s3 {
    pub mod v1 {
        tonic::include_proto!("palm.s3.v1");
    }
}

pub use self::result::{Error, GrpcResult, HttpError, HttpResult, Result};

pub const BANNER: &str = include_str!("banner.txt");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
include!(concat!(env!("OUT_DIR"), "/env.rs"));

lazy_static::lazy_static! {
    pub static ref VERSION: String = format!("{GIT_VERSION}({BUILD_TIME})");
}

// https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-WEB.md
// https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md
// https://developers.cloudflare.com/support/speed/optimization-file-size/what-will-cloudflare-compress/
pub const PROTOBUF: &str = "application/x-protobuf";
pub const FLATBUFFER: &str = "application/x-flatbuffer";

use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Environment {
    Production,
    Staging,
    #[default]
    Development,
    Testing,
}

pub fn is_stopped() -> bool {
    Path::new(".stop").exists()
}

pub fn check_config_permission<P: AsRef<Path>>(file: P) -> Result<()> {
    let file = file.as_ref();
    let mode = {
        let file = File::open(file)?;
        file.metadata()?.permissions().mode()
    };
    if ![0o100400, 0o100600].contains(&mode) {
        return Err(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some(format!("bad file ({}) mode({:#o})", file.display(), mode)),
        )));
    }
    Ok(())
}
