pub mod policy;
pub mod user;

use std::net::IpAddr;
use std::ops::Deref;

use hyper::header::{ACCEPT_LANGUAGE, AUTHORIZATION};
use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};
use tonic::{metadata::MetadataMap, Request};
use validator::Validate;

use super::super::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub peer: Option<IpAddr>,
    pub token: Option<String>,
    pub lang: String,
}

impl Session {
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
            token: Self::_authorization(mt),
            lang: Self::_accept_language(mt),
        }
    }

    fn _accept_language(mt: &MetadataMap) -> String {
        let key: &str = ACCEPT_LANGUAGE.as_ref();
        if let Some(it) = mt.get(key) {
            if let Ok(it) = it.to_str() {
                if let Ok(it) = LanguageTag::parse(it) {
                    return it.into_string();
                }
            }
        }
        "en-US".to_string()
    }

    fn _authorization(mt: &MetadataMap) -> Option<String> {
        let key: &str = AUTHORIZATION.as_ref();
        if let Some(it) = mt.get(key) {
            if let Ok(it) = it.to_str() {
                if let Some(ref it) = it.strip_prefix(Self::BEARER) {
                    return Some(it.to_string());
                }
            }
        }
        None
    }

    pub fn jwt_authorization<T>(req: &mut Request<T>, token: &str) -> Result<()> {
        let mt = req.metadata_mut();
        let key: &str = AUTHORIZATION.as_ref();
        mt.insert(key, format!("{}{}", Self::BEARER, token).parse()?);
        Ok(())
    }

    pub fn accept_language<T>(req: &mut Request<T>, lang: &LanguageTag) -> Result<()> {
        let mt = req.metadata_mut();
        let key: &str = ACCEPT_LANGUAGE.as_ref();
        let val = lang.as_str();
        mt.insert(key, val.parse()?);
        Ok(())
    }

    pub fn client_ip(&self) -> String {
        self.peer
            .map(|x| x.to_string())
            .unwrap_or_else(|| "n/a".to_string())
    }
}

#[derive(Validate)]
pub struct EmailField {
    #[validate(email, length(min = 5, max = 255))]
    pub value: String,
}

#[derive(Validate)]
pub struct StringField {
    #[validate(length(min = 1, max = 255))]
    pub value: String,
}

#[derive(Validate)]
pub struct NameField {
    #[validate(length(min = 1, max = 32))]
    pub value: String,
}

#[derive(Validate)]
pub struct PasswordField {
    #[validate(length(min = 6, max = 32))]
    pub value: String,
}

#[derive(Validate)]
pub struct TextField {
    #[validate(length(min = 1))]
    pub value: String,
}

#[derive(Validate)]
pub struct UrlField {
    #[validate(url, length(min = 1, max = 255))]
    pub value: String,
}

#[derive(Validate)]
pub struct PhoneField {
    #[validate(phone, length(min = 1, max = 255))]
    pub value: String,
}

#[derive(Validate)]
pub struct CreditCardField {
    #[validate(credit_card, length(min = 1, max = 255))]
    pub value: String,
}
