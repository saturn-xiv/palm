#[allow(clippy::missing_safety_doc, clippy::extra_unused_lifetimes)]
mod cups;
#[allow(clippy::missing_safety_doc)]
mod email;
#[allow(clippy::missing_safety_doc)]
mod sms;
#[allow(clippy::missing_safety_doc, clippy::extra_unused_lifetimes)]
mod tex;

#[allow(
    dead_code,
    unused_imports,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::useless_conversion,
    clippy::needless_borrow,
    clippy::wrong_self_convention
)]
mod palm {
    pub mod rbac {
        pub mod v1 {
            include!("rbac/generated.rs");
            include!("rbac/rbac_grpc.pb.rs");
        }
    }
    pub mod wechat_pay {
        pub mod v1 {
            include!("wechat_pay/generated.rs");
            include!("wechat_pay/wechatpay_grpc.pb.rs");
        }
    }
    pub mod loquat {
        pub mod v1 {
            include!("loquat/generated.rs");
            include!("loquat/loquat_grpc.pb.rs");
        }
    }
}

pub mod models;
pub mod schema;

pub use flatbuffers::root as flatbuffers_root;
pub use flexbuffers::{FlexbufferSerializer, Reader as FlexbufferReader};
pub use grpc::{StatusError as GrpcStatusError, client::Channel as GrpcClientChannel};
pub use protobuf::{Message as ProtobufMessage, Parse as ProtobufParse, ProtoString};

pub use cups::palm::cups::v_1 as cups_v1;
pub use email::palm::email::v_1 as email_v1;
pub use palm::loquat::v1 as loquat_v1;
pub use palm::rbac::v1 as rbac_v1;
pub use palm::wechat_pay::v1 as wechat_pay_v1;
pub use sms::palm::sms::v_1 as sms_v1;
pub use tex::palm::tex::v_1 as tex_v1;

use std::sync::Arc;

use grpc::{client::ChannelOptions, credentials::LocalChannelCredentials};

// https://grpc.io/docs/guides/custom-name-resolution/
pub fn open_grpc_channel(host: &str, port: u16) -> GrpcClientChannel {
    log::debug!("connect to http://{host}:{port}");
    GrpcClientChannel::new(
        format!("dns:///{host}:{port}"),
        Arc::new(LocalChannelCredentials::new()),
        ChannelOptions::default(),
    )
}
