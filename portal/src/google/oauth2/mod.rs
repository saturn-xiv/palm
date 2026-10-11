pub mod scopes;

use hyacinth::oauth2_v1::{
    GoogleFetchTokenRequest, GoogleFetchTokenResponse, GoogleRevokeResponse,
    GoogleSignInUrlRequest, GoogleSignInUrlResponse, GoogleUserInfoResponse,
    google_fetch_token_response::Credentials,
};

use super::super::{Dahlia, HttpError, Result};

impl Dahlia {
    pub async fn google_oauth2_sign_in_url<A: Into<String>>(
        &self,
        scopes: Vec<A>,
        redirect_uri: &str,
    ) -> Result<GoogleSignInUrlResponse> {
        let mut req = GoogleSignInUrlRequest::default();
        req.set_redirect_uri(redirect_uri);
        for it in scopes {
            let it = it.into();
            req.scopes_mut().push(it);
        }
        let res = self
            .google_oauth2
            .sign_in_url(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(res)
    }
    pub async fn google_oauth2_fetch_token<A: Into<String>>(
        &self,
        authorization_response: &str,
        scopes: Vec<A>,
        state: &str,
    ) -> Result<GoogleFetchTokenResponse> {
        let mut req = GoogleFetchTokenRequest::default();
        req.set_authorization_response(authorization_response);
        req.set_state(state);
        for it in scopes {
            let it = it.into();
            req.scopes_mut().push(it);
        }
        let res = self
            .google_oauth2
            .fetch_token(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(res)
    }
    pub async fn google_oauth2_user_info(
        &self,
        credentials: Credentials,
    ) -> Result<GoogleUserInfoResponse> {
        let res = self
            .google_oauth2
            .user_info(credentials)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(res)
    }

    pub async fn google_oauth2_revoke(
        &self,
        credentials: Credentials,
    ) -> Result<GoogleRevokeResponse> {
        let res = self
            .google_oauth2
            .revoke(credentials)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(res)
    }
}
