use std::ops::Deref;
use std::process::Command;
use std::sync::Arc;

use phlox::{GrpcResult, Regex, Result, cache::redis::SingleClient as RedisClient, jwt::JwtHS512};
use tonic::{Request, Response, Status};

use super::super::super::super::palm::cups::v1::{
    CupsPrintersResponse, cups_printers_response::Item as PrinterStatusItem, cups_server::Cups,
};
use super::current_user;

pub struct Server {
    pub jwt: Arc<JwtHS512>,
    pub redis: Arc<RedisClient>,
}

#[tonic::async_trait]
impl Cups for Server {
    async fn printers(&self, request: Request<()>) -> GrpcResult<CupsPrintersResponse> {
        let jwt = self.jwt.deref();
        let cache = self.redis.deref();
        let _ = current_user(jwt, cache, &request)
            .map_err(|e| Status::permission_denied(e.to_string()))?;

        Ok(Response::new(CupsPrintersResponse {
            items: PrinterStatusItem::list().map_err(|e| Status::internal(e.to_string()))?,
        }))
    }
}

impl PrinterStatusItem {
    pub fn list() -> Result<Vec<Self>> {
        let re = Regex::new(r"printer (?<name>\w+) is (?<status>\w+).  (?<detail>[[:ascii:]]+)")?;
        let mut items = Vec::new();
        {
            for line in String::from_utf8(
                Command::new("sh")
                    .arg("-c")
                    .arg("lpstat -p -l")
                    .output()?
                    .stdout,
            )?
            .lines()
            {
                if let Some(caps) = re.captures(line) {
                    items.push(Self {
                        name: caps["name"].to_string(),
                        status: caps["status"].to_string(),
                        detail: caps["detail"].to_string(),
                    });
                }
            }
        }
        Ok(items)
    }
}
