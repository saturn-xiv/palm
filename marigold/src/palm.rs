pub mod crypto {
    pub mod v1 {
        tonic::include_proto!("palm.crypto.v1");
    }
}
pub mod email {
    pub mod v1 {
        tonic::include_proto!("palm.email.v1");
    }
}
pub mod sms {
    pub mod v1 {
        tonic::include_proto!("palm.sms.v1");
    }
}
pub mod tex {
    pub mod v1 {
        tonic::include_proto!("palm.tex.v1");
    }
}
pub mod s3 {
    pub mod v1 {
        tonic::include_proto!("palm.s3.v1");
    }
}
pub mod rbac {
    pub mod v1 {
        tonic::include_proto!("palm.rbac.v1");
    }
}
pub mod portal {
    pub mod v1 {
        tonic::include_proto!("palm.portal.v1");
    }
}

pub mod accounting {
    pub mod v1 {
        tonic::include_proto!("palm.accounting.v1");
    }
}
pub mod blog {
    pub mod v1 {
        tonic::include_proto!("palm.blog.v1");
    }
}
pub mod cms {
    pub mod v1 {
        tonic::include_proto!("palm.cms.v1");
    }
}
pub mod forum {
    pub mod v1 {
        tonic::include_proto!("palm.forum.v1");
    }
}

pub mod wechatpay {
    pub mod v1 {
        tonic::include_proto!("palm.wechatpay.v1");
    }
}
