pub mod fetch_access_token_with_auth_code;

use std::path::PathBuf;

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::{Deserialize, Serialize};

// https://developers.google.com/identity/protocols/oauth2/web-server#httprest
// openssl ecparam -name prime256v1 -genkey -noout -out dpop_private.pem
// openssl ec -in dpop_private.pem -pubout -out dpop_public.pem
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "client-id")]
    pub client_id: String,
    #[serde(rename = "client-secret")]
    pub client_secret: String,
    #[serde(rename = "redirect-uri")]
    pub redirect_uri: String,
    #[serde(rename = "dpop-private-pem")]
    pub dpop_private_file: PathBuf,
    #[serde(rename = "redirect-public-pem")]
    pub dpop_public_file: PathBuf,
}

impl Config {
    pub fn sign_in_url(&self, state: &str) -> String {
        let scopes = [Self::SCOPE_OPENID, Self::SCOPE_PROFILE, Self::SCOPE_EMAIL];
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&scope={}&access_type=offline&include_granted_scopes=true&response_type=code&state={}&redirect_uri={}",
            self.client_id,
            utf8_percent_encode(&scopes.join(" "), NON_ALPHANUMERIC),
            state,
            utf8_percent_encode(&self.redirect_uri, NON_ALPHANUMERIC)
        )
    }

    // https://developers.google.com/identity/protocols/oauth2/scopes#oauth2
    const SCOPE_OPENID: &str = "openid";
    const SCOPE_PROFILE: &str = "https://www.googleapis.com/auth/userinfo.profile";
    const SCOPE_EMAIL: &str = "https://www.googleapis.com/auth/userinfo.email";
}
