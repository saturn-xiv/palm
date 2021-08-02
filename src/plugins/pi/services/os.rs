use prost_types::Duration;
use tonic::{Request, Response, Status};

use super::super::super::nut::services::Session;

pub struct Service {}

#[tonic::async_trait]
impl super::os_server::Os for Service {
    async fn uptime(&self, req: Request<()>) -> Result<Response<Duration>, Status> {
        let ss = Session::new(&req)
            .map_err(|x| x.to_string())
            .map_err(Status::unauthenticated)?;
        debug!("got a request from {:?}", ss);
        let si = nix::sys::sysinfo::sysinfo()
            .map_err(|x| x.to_string())
            .map_err(Status::internal)?;
        let dur = si.uptime();
        Ok(Response::new(dur.into()))
    }
}
