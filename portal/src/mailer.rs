use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};

use lettre::SmtpTransport;

use super::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Smtp {
    pub host: String,
    pub user: String,
    pub password: String,
}

impl Smtp {
    pub fn open(&self) -> Result<SmtpTransport> {
        let mailer = SmtpTransport::relay(&self.host)?
            .credentials(Credentials::new(self.user.clone(), self.password.clone()))
            .build();
        Ok(mailer)
    }
}
