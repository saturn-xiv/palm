#[allow(clippy::missing_safety_doc)]
mod email;
#[allow(clippy::missing_safety_doc, clippy::extra_unused_lifetimes)]
mod tex;

pub use email::palm::email::v_1 as email_v1;
pub use tex::palm::tex::v_1 as tex_v1;

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_imports,
    clippy::useless_conversion
)]
pub mod palm {
    pub mod casbin {
        pub mod v1 {
            include!("casbin.u.pb.rs");
            include!("casbin_grpc.pb.rs");
        }
    }
}
