#[allow(clippy::missing_safety_doc)]
mod email;
#[allow(clippy::missing_safety_doc, clippy::extra_unused_lifetimes)]
mod tex;

#[allow(
    clippy::derivable_impls,
    clippy::useless_conversion,
    clippy::unnecessary_fallible_conversions
)]
pub mod google {
    pub mod protobuf {
        include!("google/protobuf/generated.rs");
    }
}

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_imports,
    clippy::useless_conversion
)]
mod palm {
    pub mod casbin {
        pub mod v1 {
            include!("casbin/generated.rs");
            include!("casbin/casbin_grpc.pb.rs");
        }
    }
    // pub mod portal {
    //     pub mod v1 {
    //         include!("portal.u.pb.rs");
    //         include!("portal_grpc.pb.rs");
    //     }
    // }
}

pub use email::palm::email::v_1 as email_v1;
pub use palm::casbin::v1 as casbin_v1;
pub use tex::palm::tex::v_1 as tex_v1;
