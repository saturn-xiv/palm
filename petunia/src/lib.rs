pub const BANNER: &str = include_str!("banner.txt");

include!(concat!(env!("OUT_DIR"), "/env.rs"));

pub mod client;

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
