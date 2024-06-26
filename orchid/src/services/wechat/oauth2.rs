use std::sync::Arc;

use palm::{
    cache::redis::Pool as RedisPool,
    orchid::v1,
    try_grpc,
    wechat::{
        oauth2::{Client as Oauth2Client, Oauth2 as Oauth2Config},
        Client as WechatClient, Config as WechatConfig,
    },
    GrpcResult,
};
use tonic::{Request, Response};

use super::super::super::env::Config;

pub struct Service {
    pub redis: RedisPool,
    pub config: Arc<Config>,
}

#[tonic::async_trait]
impl v1::wechat_oauth2_server::WechatOauth2 for Service {
    async fn qr_connect(
        &self,
        req: Request<v1::WechatOauth2QrConnectRequest>,
    ) -> GrpcResult<v1::WechatOauth2QrConnectResponse> {
        let req = req.into_inner();
        let cfg = try_grpc!(self.config.wechat(&req.app_id))?;
        let lang = req.language().to_string();
        let url = try_grpc!(cfg.qr_connect(&req.redirect_uri, &req.state, &lang))?;

        Ok(Response::new(v1::WechatOauth2QrConnectResponse { url }))
    }
    async fn login(
        &self,
        req: Request<v1::WechatOauth2LoginRequest>,
    ) -> GrpcResult<v1::WechatOauth2LoginResponse> {
        let req = req.into_inner();
        debug!("wechat oauth2 login {:?}", req);
        let cli = WechatClient {
            redis: self.redis.clone(),
            config: try_grpc!(self.config.wechat(&req.app_id))?,
        };
        let lang = req.language().to_string();
        let (access_token, openid) = try_grpc!(cli.access_token_by_code(&req.code).await)?;
        let it = try_grpc!(WechatConfig::userinfo(&access_token, &openid, &lang).await)?;

        Ok(Response::new(v1::WechatOauth2LoginResponse {
            openid: it.openid.clone(),
            unionid: it.unionid.clone(),
            nickname: it.nickname.clone(),
            province: it.province.clone(),
            city: it.city.clone(),
            country: it.country.clone(),
            headimgurl: it.headimgurl.clone(),
            privilege: it.privilege.clone(),
            sex: it.sex,
        }))
    }
}
