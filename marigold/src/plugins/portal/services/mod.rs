pub mod cups;

use phlox::{Jwt, Result, cache::ProtobufCacher, grpc::verify as grpc_verify};
use tonic::Request;

use super::super::super::palm::portal::v1::Session;
use super::ISSUER;

pub fn current_user<J: Jwt, C: ProtobufCacher, T>(
    jwt: &J,
    cache: &C,
    request: &Request<T>,
) -> Result<Session> {
    let subject = grpc_verify(jwt, request, ISSUER, &[WEB_USER_SIGN_IN_AUDIENCE])?;
    let ss = cache.get(subject)?;
    Ok(ss)
}

pub const WEB_USER_SIGN_IN_AUDIENCE: &str = "web.user-sign-in";
