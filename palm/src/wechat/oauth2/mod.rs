pub mod access_token;
pub mod get_api_domain_ip;
pub mod message_push;
pub mod qr_connect;
pub mod refresh_token;
pub mod userinfo;

use std::any::type_name;
use std::fmt::Display;
use std::ops::DerefMut;

use redis::{cluster::ClusterConnection as RedisClusterConnection, Commands};
use reqwest::Client as HttpClient;

use super::super::Result;
use super::{Config, Query};

pub trait Oauth2 {
    // https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Get_the_WeChat_server_IP_address.html
    fn get_api_domain_ip(
        access_token: &str,
    ) -> impl std::future::Future<Output = Result<get_api_domain_ip::Response>> + Send;
    // https://developers.weixin.qq.com/doc/oplatform/Website_App/WeChat_Login/Wechat_Login.html
    fn access_token(
        &self,
        code: &str,
    ) -> impl std::future::Future<Output = Result<access_token::Response>> + Send;
    fn refresh_token(
        &self,
        code: &str,
    ) -> impl std::future::Future<Output = Result<refresh_token::Response>> + Send;
    fn userinfo(
        access_token: &str,
        openid: &str,
        lang: &str,
    ) -> impl std::future::Future<Output = Result<userinfo::Response>> + Send;
    // https://developers.weixin.qq.com/doc/oplatform/Website_App/WeChat_Login/Wechat_Login.html
    fn qr_connect<S: Display>(&self, redirect_uri: &str, state: &S, lang: &str) -> Result<String>;
}

impl Oauth2 for Config {
    async fn get_api_domain_ip(access_token: &str) -> Result<get_api_domain_ip::Response> {
        let client = HttpClient::new();
        let response = client
            .get(Self::url("/cgi-bin/get_api_domain_ip"))
            .query(&Query {
                access_token: access_token.to_string(),
            })
            .send()
            .await?;
        let response = Self::body(response).await?;
        Ok(response)
    }
    async fn access_token(&self, code: &str) -> Result<access_token::Response> {
        let client = HttpClient::new();
        let response = client
            .get(Self::url("/sns/oauth2/access_token"))
            .query(&access_token::Query {
                appid: self.app_id.clone(),
                secret: self.app_secret.clone(),
                code: code.to_string(),
                grant_type: access_token::Query::GRANT_TYPE.to_string(),
            })
            .send()
            .await?;
        let response = Self::body(response).await?;
        Ok(response)
    }
    async fn refresh_token(&self, refresh_token: &str) -> Result<refresh_token::Response> {
        let client = HttpClient::new();
        let response = client
            .get(Self::url("/sns/oauth2/refresh_token"))
            .query(&refresh_token::Query {
                appid: self.app_id.clone(),
                refresh_token: refresh_token.to_string(),
                grant_type: access_token::Query::GRANT_TYPE.to_string(),
            })
            .send()
            .await?;
        let response = Self::body(response).await?;
        Ok(response)
    }

    async fn userinfo(access_token: &str, openid: &str, lang: &str) -> Result<userinfo::Response> {
        let client = HttpClient::new();
        let response = client
            .get(Self::url("/sns/userinfo"))
            .query(&userinfo::Query {
                access_token: access_token.to_string(),
                openid: openid.to_string(),
                lang: lang.to_string(),
            })
            .send()
            .await?;
        let response = Self::body(response).await?;
        Ok(response)
    }

    fn qr_connect<S: Display>(&self, redirect_uri: &str, state: &S, lang: &str) -> Result<String> {
        qr_connect::url(&self.app_id, redirect_uri, state, lang)
    }
}

pub trait Client {
    fn access_token_by_code(
        &self,
        code: &str,
    ) -> impl std::future::Future<Output = Result<(String, String)>> + Send;
    fn access_token_by_openid(
        &self,
        openid: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send;
    fn refresh(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    fn access_token_key(&self, openid: &str) -> String;
    fn refresh_token_key(&self, openid: &str) -> String;
    fn set_tokens(
        &self,
        ch: &mut RedisClusterConnection,
        openid: &str,
        access_token: &str,
        expires_in: u64,
        refresh_token: &str,
    ) -> Result<()>;
}

impl Client for super::Client {
    async fn access_token_by_code(&self, code: &str) -> Result<(String, String)> {
        let response = self.config.access_token(code).await?;
        let mut ch = self.redis.get()?;
        let ch = ch.deref_mut();
        self.set_tokens(
            ch,
            &response.openid,
            &response.access_token,
            response.expires_in,
            &response.refresh_token,
        )?;
        Ok((response.access_token, response.openid))
    }
    async fn access_token_by_openid(&self, openid: &str) -> Result<String> {
        let access_token_key = self.access_token_key(openid);

        let mut ch = self.redis.get()?;
        let ch = ch.deref_mut();

        if let Ok(it) = Commands::get::<&String, String>(ch, &access_token_key) {
            return Ok(it);
        }

        let refresh_token_key = self.refresh_token_key(openid);
        let refresh_token = Commands::get::<&String, String>(ch, &refresh_token_key)?;
        let response = self.config.refresh_token(&refresh_token).await?;
        self.set_tokens(
            ch,
            &response.openid,
            &response.access_token,
            response.expires_in,
            &response.refresh_token,
        )?;
        Ok(response.access_token)
    }
    fn access_token_key(&self, openid: &str) -> String {
        format!(
            "{}://{}/access-token/{}",
            type_name::<Self>(),
            self.config.app_id,
            openid
        )
    }
    fn refresh_token_key(&self, openid: &str) -> String {
        format!(
            "{}://{}/refresh-token/{}",
            type_name::<Self>(),
            self.config.app_id,
            openid
        )
    }

    fn set_tokens(
        &self,
        cache: &mut RedisClusterConnection,
        openid: &str,
        access_token: &str,
        expires_in: u64,
        refresh_token: &str,
    ) -> Result<()> {
        let access_token_key = self.access_token_key(openid);
        let refresh_token_key = self.refresh_token_key(openid);

        Commands::set_ex(cache, &access_token_key, access_token, expires_in - 60)?;
        Commands::set_ex(
            cache,
            &refresh_token_key,
            refresh_token,
            60 * 60 * 24 * (30 - 1),
        )?;
        Ok(())
    }

    async fn refresh(&self) -> Result<()> {
        // TODO renew refresh token every 29 days
        todo!()
    }
}
