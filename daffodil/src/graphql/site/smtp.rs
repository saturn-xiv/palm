use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Validate, Deserialize, Debug, Clone)]
pub struct Profile {
    #[validate(length(min = 2, max = 63))]
    pub host: String,
    pub port: u16,
    #[validate(email, length(min = 2, max = 31))]
    pub account: String,
    #[validate(length(min = 6, max = 63))]
    pub password: String,
}

#[derive(GraphQLObject)]
#[graphql(name = "GetSiteSmtpResponse")]
pub struct Show {
    pub host: String,
    pub port: i32,
    pub account: String,
}

impl From<Profile> for Show {
    fn from(it: Profile) -> Self {
        Self {
            host: it.host.clone(),
            account: it.account.clone(),
            port: it.port as i32,
        }
    }
}
