#[allow(clippy::missing_safety_doc)]
mod email;
#[allow(clippy::missing_safety_doc, clippy::extra_unused_lifetimes)]
mod tex;

pub use email::palm::email::v_1 as email_v1;
pub use tex::palm::tex::v_1 as tex_v1;
