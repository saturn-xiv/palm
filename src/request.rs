use std::collections::HashMap;
use std::fmt::Debug;

use actix_web::{
    dev,
    error::{ErrorBadRequest, ErrorUnauthorized},
    http::HeaderMap,
    Error, FromRequest, HttpRequest,
};
use cookie::Cookie;
use futures_util::future::{err, ok, Ready};
use hyper::header::{ACCEPT_LANGUAGE, AUTHORIZATION, COOKIE};
use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Locale(pub String);

impl Locale {
    fn detect(req: &HttpRequest) -> Option<LanguageTag> {
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
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let lang = Self::detect(req)
            .map(|x| x.to_string())
            .unwrap_or_else(|| "en-US".to_string());
        ok(Self(lang))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientIp(pub String);

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
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        match Self::detect(req.headers()) {
            Some(it) => ok(Self(it)),
            None => err(ErrorBadRequest("can't detect peer ip")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token(pub String);

impl Token {
    fn detect(req: &HttpRequest) -> Option<String> {
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
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        match Self::detect(req) {
            Some(it) => ok(Self(it)),
            None => err(ErrorUnauthorized("can't detect token")),
        }
    }
}
