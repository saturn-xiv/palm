pub mod openssl;

use std::ops::Add;

use chrono::{Datelike, Duration, NaiveDateTime, Utc};
use hyper::{header::AUTHORIZATION, http::StatusCode};
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataValue},
    Request as GrpcRequest,
};
use uuid::Uuid;

use super::{HttpError, Result};

pub const BEARER: &str = "Bearer ";

// https://www.ibm.com/support/knowledgecenter/zh/SSEQTP_8.5.5/com.ibm.websphere.wlp.doc/ae/cwlp_jwttoken.html
// https://jwt.io/
// https://tools.ietf.org/html/rfc7519
pub trait Jwt {
    fn verify(&self, token: &str, issuer: &str, audience: &str) -> Result<(String, String)>;
    fn sign(
        &self,
        jwt_id: &str,
        issuer: &str,
        subject: &str,
        audience: &str,
        timestamps: (NaiveDateTime, NaiveDateTime, NaiveDateTime),
    ) -> Result<String>;

    fn sign_by_duration(
        &self,
        issuer: &str,
        subject: &str,
        audience: &str,
        ttl: Duration,
    ) -> Result<String> {
        let (iat, nbf, exp) = Self::timestamps(ttl)?;
        self.sign(
            &Uuid::new_v4().to_string(),
            issuer,
            subject,
            audience,
            (iat, nbf, exp),
        )
    }
    fn sign_by_range(
        &self,
        issuer: &str,
        subject: &str,
        audience: &str,
        not_before: NaiveDateTime,
        expired_at: NaiveDateTime,
    ) -> Result<String> {
        self.sign(
            &Uuid::new_v4().to_string(),
            issuer,
            subject,
            audience,
            (Utc::now().naive_local(), not_before, expired_at),
        )
    }

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

    fn timestamps(ttl: Duration) -> Result<(NaiveDateTime, NaiveDateTime, NaiveDateTime)> {
        let now = Utc::now();
        let nbf = now.add(Duration::try_seconds(-1).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad seconds".to_string()),
        )))?);
        let exp = now.add(ttl);
        Ok((now.naive_local(), nbf.naive_local(), exp.naive_local()))
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
