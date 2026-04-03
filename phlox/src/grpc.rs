use hyper::StatusCode;
use tonic::Request;

use super::{HttpError, Jwt, Result, headers::AUTHORIZATION};

pub fn verify<J: Jwt, T, A: ToString>(
    jwt: &J,
    request: &Request<T>,
    issuer: &str,
    audiences: &[A],
) -> Result<String> {
    let mt = request.metadata();
    match mt.get(AUTHORIZATION.to_lowercase()) {
        Some(auth) => {
            let auth = auth.to_str()?;
            match auth.strip_prefix(J::BEARER) {
                Some(token) => jwt.verify(token, issuer, audiences),
                None => Err(Box::new(HttpError(
                    StatusCode::FORBIDDEN,
                    Some("invalid auth header".to_string()),
                ))),
            }
        }
        None => Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some("empty auth header".to_string()),
        ))),
    }
}
