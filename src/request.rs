use std::collections::HashMap;
use std::fmt::Debug;
use std::result::Result as StdResult;

use cookie::Cookie;
use hyper::{
    header::{ACCEPT_LANGUAGE, AUTHORIZATION, COOKIE},
    Body, HeaderMap, Request,
};
use language_tags::LanguageTag;
use url::Url;

pub trait FromRequest: Sync + Send {
    type Error: Sync + Send + Debug;
    type Item: Sync + Send;
    fn from_request(req: &Request<Body>) -> StdResult<Self::Item, Self::Error>;
}

pub struct Locale(pub String);

impl Locale {
    fn detect(req: &Request<Body>) -> Option<LanguageTag> {
        let key = "locale";

        // 1. Check URL arguments.
        {
            if let Ok(it) = Url::parse(&req.uri().to_string()) {
                let query: HashMap<String, String> = it.query_pairs().into_owned().collect();
                for (k, v) in query {
                    if k == key {
                        if let Ok(it) = LanguageTag::parse(&v) {
                            return Some(it);
                        }
                    }
                }
            }
        }

        // 2. Get language information from cookies.
        for it in req.headers().get_all(COOKIE) {
            if let Ok(it) = it.to_str() {
                if let Ok(it) = Cookie::parse_encoded(it) {
                    if it.name() == key {
                        if let Ok(it) = LanguageTag::parse(it.value()) {
                            return Some(it);
                        }
                    }
                }
            }
        }

        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4
        if let Some(it) = req.headers().get(ACCEPT_LANGUAGE) {
            if let Ok(it) = it.to_str() {
                if let Ok(it) = it.parse::<LanguageTag>() {
                    return Some(it);
                }
            }
        }

        None
    }
}

impl FromRequest for Locale {
    type Error = ();
    type Item = Self;
    fn from_request(req: &Request<Body>) -> StdResult<Self, Self::Error> {
        let lang = Self::detect(req)
            .map(|x| x.to_string())
            .unwrap_or_else(|| "en-US".to_string());
        Ok(Self(lang))
    }
}

pub struct ClientIp(pub Option<String>);

impl ClientIp {
    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For
    /// https://github.com/gin-gonic/gin/blob/893c6cae07ef564cbdd2796589c449dd2ac87d21/context.go#L651
    fn detect(headers: &HeaderMap) -> Option<String> {
        if let Some(it) = headers.get("X-Forwarded-For") {
            if let Ok(it) = it.to_str() {
                let items: Vec<&str> = it.split(',').collect();
                if let Some(it) = items.first() {
                    let it = it.trim();
                    if !it.is_empty() {
                        return Some(it.to_string());
                    }
                }
            }
        }
        if let Some(it) = headers.get("X-Real-Ip") {
            if let Ok(it) = it.to_str() {
                let it = it.trim();
                if !it.is_empty() {
                    return Some(it.to_string());
                }
            }
        }
        if let Some(it) = headers.get("X-Appengine-Remote-Addr") {
            if let Ok(it) = it.to_str() {
                let it = it.trim();
                if !it.is_empty() {
                    return Some(it.to_string());
                }
            }
        }
        None
    }
}

impl FromRequest for ClientIp {
    type Error = ();
    type Item = Self;

    fn from_request(req: &Request<Body>) -> StdResult<Self, Self::Error> {
        let it = Self::detect(req.headers());
        Ok(Self(it))
    }
}

pub struct Token(pub Option<String>);

impl Token {
    fn detect(req: &Request<Body>) -> Option<String> {
        let key = "token";
        // 1. Check header
        {
            let headers = req.headers();
            if let Some(it) = headers.get(AUTHORIZATION) {
                if let Ok(it) = it.to_str() {
                    if let Some(ref it) = it.strip_prefix("Bearer ") {
                        return Some(it.to_string());
                    }
                }
            }
        }

        // 2. Check URL arguments.
        {
            if let Ok(it) = Url::parse(&req.uri().to_string()) {
                let query: HashMap<String, String> = it.query_pairs().into_owned().collect();
                for (k, v) in query {
                    if k == key {
                        return Some(v);
                    }
                }
            }
        }
        // 3. Get token information from cookies.
        for it in req.headers().get_all(COOKIE) {
            if let Ok(it) = it.to_str() {
                if let Ok(it) = Cookie::parse_encoded(it) {
                    if it.name() == key {
                        return Some(it.to_string());
                    }
                }
            }
        }
        None
    }
}

impl FromRequest for Token {
    type Error = ();
    type Item = Self;
    fn from_request(req: &Request<Body>) -> StdResult<Self, Self::Error> {
        let it = Self::detect(req);
        Ok(Self(it))
    }
}
