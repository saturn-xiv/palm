use std::sync::Arc;
use std::time::SystemTime;

use phlox::{GrpcResult, cache::redis::SingleClient as RedisClient, jwt::JwtHS512};
use prost_types::Timestamp;
use tonic::{Request, Response};

use super::super::super::super::{
    BUILD_TIME, GIT_VERSION,
    palm::portal::v1::{SiteHeartbeatResponse, site_server::Site},
};

pub struct Server {
    pub jwt: Arc<JwtHS512>,
    pub redis: Arc<RedisClient>,
}

#[tonic::async_trait]
impl Site for Server {
    async fn heartbeat(&self, _request: Request<()>) -> GrpcResult<SiteHeartbeatResponse> {
        Ok(Response::new(SiteHeartbeatResponse {
            version: format!("{}({})", GIT_VERSION, BUILD_TIME),
            created_at: Some(Timestamp::from(SystemTime::now())),
        }))
    }
}
