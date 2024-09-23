pub mod cache;
pub mod crypto;
pub mod grpc;
pub mod jwt;
pub mod queue;
pub mod result;

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
pub mod wechat {
    pub mod v1 {
        tonic::include_proto!("palm.wechat.v1");
    }
}

pub use self::result::{Error, GrpcResult, HttpError, HttpResult, Result};

pub const BANNER: &str = include_str!("banner.txt");
include!(concat!(env!("OUT_DIR"), "/env.rs"));

lazy_static::lazy_static! {
    pub static ref VERSION: String = format!("{GIT_VERSION}({BUILD_TIME})");
}

// https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-WEB.md
// https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md
// https://developers.cloudflare.com/support/speed/optimization-file-size/what-will-cloudflare-compress/
pub const PROTOBUF: &str = "application/x-protobuf";
pub const FLATBUFFER: &str = "application/x-flatbuffer";
