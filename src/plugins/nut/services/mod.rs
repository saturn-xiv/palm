pub mod site;
pub mod user;

pub mod api {
    tonic::include_proto!("palm.nut.v1");
}

use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use tonic::{metadata::MetadataMap, Request};

use super::super::super::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub peer: Option<IpAddr>,
    pub token: Option<String>,
}

impl Session {
    const AUTHORIZAATION: &'static str = "authorization";
    const BEARER: &'static str = "Bearer ";

    pub fn new<T>(req: &Request<T>) -> Self {
        let mt = req.metadata();
        // for it in mt.iter() {
        //     if let KeyAndValueRef::Ascii(ref key, val) = it {
        //         debug!("{} => {}", key, val.to_str()?);
        //     }
        // }
        Self {
            peer: req.remote_addr().map(|x| x.ip()),
            token: Self::authorization(mt),
        }
    }

    fn authorization(mt: &MetadataMap) -> Option<String> {
        if let Some(it) = mt.get(Self::AUTHORIZAATION) {
            if let Ok(it) = it.to_str() {
                if let Some(ref it) = it.strip_prefix(Self::BEARER) {
                    return Some(it.to_string());
                }
            }
        }
        None
    }

    pub fn auth<T>(req: &mut Request<T>, token: &str) -> Result<()> {
        let mt = req.metadata_mut();
        mt.insert(
            Self::AUTHORIZAATION,
            format!("{}{}", Self::BEARER, token).parse()?,
        );

        Ok(())
    }
}
