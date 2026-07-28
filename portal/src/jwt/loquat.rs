use chrono::{Duration, Utc};
use hyacinth::loquat_v1::{JwtSignRequest, JwtVerifyRequest};
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use super::super::{HttpError, Jwt, Loquat, Result};

impl Jwt for Loquat {
    async fn sign<A: Into<String>, P: Serialize>(
        &self,
        issuer: &str,
        subject: &str,
        audiences: Vec<A>,
        ttl: Duration,
        payload: Option<P>,
    ) -> Result<String> {
        let now = Utc::now();
        let mut req = JwtSignRequest::default();
        req.set_jwt_id(Uuid::new_v4().to_string());
        req.set_subject(subject);
        req.set_issuer(issuer);
        req.set_issued_at(now.timestamp());
        req.set_not_before(now.timestamp());
        req.set_expired_at((now + ttl).timestamp());
        for it in audiences {
            let it = it.into();
            req.audiences_mut().push(it);
        }
        if let Some(ref payload) = payload {
            let payload = flexbuffers::to_vec(payload)?;
            req.set_payload(payload);
        }
        let res = self
            .jwt
            .sign(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(res.token().to_string())
    }
    async fn verify<P: DeserializeOwned>(
        &self,
        token: &str,
        issuer: &str,
        audience: &str,
    ) -> Result<(String, Option<P>)> {
        let mut req = JwtVerifyRequest::default();
        req.set_token(token);
        req.set_issuer(issuer);
        req.set_audience(audience);
        let res = self
            .jwt
            .verify(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        let payload = if res.has_payload() {
            let it = flexbuffers::from_slice(res.payload())?;
            Some(it)
        } else {
            None
        };
        Ok((res.subject().to_string(), payload))
    }
}
