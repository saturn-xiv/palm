pub mod oauth;

pub mod v1 {
    tonic::include_proto!("palm.camelia.v1");
}

impl v1::SiteSetInfoRequest {
    pub const TITLE: &'static str = "site.title";
    pub const SUBHEAD: &'static str = "site.subhead";
    pub const DESCRIPTION: &'static str = "site.description";
    pub const COPYRIGHT: &'static str = "site.copyright";
}
