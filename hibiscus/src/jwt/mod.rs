pub mod openssl;

use std::ops::Add;

use chrono::{Datelike, Duration, NaiveDateTime, Utc};
use hyper::{header::AUTHORIZATION, http::StatusCode};
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataValue},
    Request as GrpcRequest,
};

use super::{HttpError, Result};

pub const BEARER: &str = "Bearer ";

// https://www.ibm.com/support/knowledgecenter/zh/SSEQTP_8.5.5/com.ibm.websphere.wlp.doc/ae/cwlp_jwttoken.html
// https://jwt.io/
// https://tools.ietf.org/html/rfc7519
pub trait Jwt {
    fn sign_by_duration(
        &self,
        issuer: &str,
        subject: &str,
        audience: &str,
        ttl: Duration,
    ) -> Result<String>;
    fn sign_by_range(
        &self,
        issuer: &str,
        subject: &str,
        audience: &str,
        not_before: NaiveDateTime,
        expiration_time: NaiveDateTime,
    ) -> Result<String>;
    fn verify(&self, token: &str, issuer: &str, audience: &str) -> Result<(String, String)>;

    fn bearer(token: &str) -> String {
        format!("{}{}", BEARER, token)
    }
    fn authorization<R>(request: &mut GrpcRequest<R>, token: &str) -> Result<()> {
        let val: MetadataValue<Ascii> = {
            let it = Self::bearer(token);
            it.parse()?
        };

        let key: MetadataKey<Ascii> = {
            let it = AUTHORIZATION.as_str();
            let it = it.to_lowercase();
            it.parse()?
        };
        request.metadata_mut().insert(key, val);

        debug!("request header {:?}", request.metadata());
        Ok(())
    }

    fn timestamps(ttl: Duration) -> Result<(i64, i64, i64)> {
        let now = Utc::now();
        let nbf = now.add(Duration::try_seconds(-1).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad seconds".to_string()),
        )))?);
        let exp = now.add(ttl);
        Ok((now.timestamp(), nbf.timestamp(), exp.timestamp()))
    }
    fn years(y: i32) -> Result<(i64, i64, i64)> {
        let now = Utc::now();
        let nbf = now.add(Duration::try_seconds(-1).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad seconds".to_string()),
        )))?);
        let exp = now.with_year(nbf.year() + y).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad years".to_string()),
        )))?;
        Ok((now.timestamp(), nbf.timestamp(), exp.timestamp()))
    }
}
