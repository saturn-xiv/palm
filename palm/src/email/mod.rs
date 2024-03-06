use hyper::StatusCode;
use lettre::{
    message::{Attachment, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{HttpError, Result};

pub mod v1 {
    tonic::include_proto!("palm.email.v1");
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct Address {
    #[validate(length(min = 1, max = 31))]
    pub name: String,
    #[validate(email, length(min = 1, max = 63))]
    pub email: String,
}
impl Address {
    pub fn mailbox(&self) -> Result<Mailbox> {
        self.validate()?;
        let it = Mailbox {
            name: Some(self.name.clone()),
            email: self.email.parse()?,
        };
        Ok(it)
    }
}

impl From<v1::send_email_task::Address> for Address {
    fn from(it: v1::send_email_task::Address) -> Self {
        Self {
            name: it.name.clone(),
            email: it.email.clone(),
        }
    }
}

impl v1::send_email_task::Address {
    pub fn mailbox(&self) -> Result<Mailbox> {
        let it: Address = self.clone().into();
        it.mailbox()
    }
}

// https://support.google.com/mail/answer/7126229#zippy=%2Cstep-change-smtp-other-settings-in-your-email-client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub password: String,
    pub from: Address,
    pub cc: Vec<Address>,
    pub bcc: Vec<Address>,
}

impl Config {
    pub fn send(&self, task: &v1::SendEmailTask) -> Result<()> {
        let account = self.from.mailbox()?;
        let to = task
            .to
            .as_ref()
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("empty to address".to_string()),
            )))?
            .mailbox()?;

        let msg = {
            let mut builder = Message::builder()
                .from(account.clone())
                .reply_to(account)
                .to(to)
                .subject(&task.subject);
            // cc
            {
                for it in self.cc.iter() {
                    builder = builder.cc(it.mailbox()?);
                }
                for it in task.cc.iter() {
                    builder = builder.cc(it.mailbox()?);
                }
            }
            // bcc
            {
                for it in self.bcc.iter() {
                    builder = builder.bcc(it.mailbox()?);
                }
                for it in task.bcc.iter() {
                    builder = builder.bcc(it.mailbox()?);
                }
            }
            // attachments
            let mut part = MultiPart::alternative().build();
            // add message body
            if let Some(ref body) = task.body {
                if body.html {
                    part = part.singlepart(SinglePart::html(body.payload.clone()));
                } else {
                    part = part.singlepart(SinglePart::plain(body.payload.clone()));
                }
            }

            for it in task.attachments.iter() {
                part = if it.inline {
                    part.singlepart(
                        Attachment::new_inline(it.title.clone())
                            .body(it.payload.to_vec(), it.content_type.parse()?),
                    )
                } else {
                    part.singlepart(
                        Attachment::new(it.title.clone())
                            .body(it.payload.to_vec(), it.content_type.parse()?),
                    )
                };
            }

            builder.multipart(part)?
        };

        let mailer = SmtpTransport::relay(&self.host)?
            .credentials(Credentials::new(
                self.from.email.clone(),
                self.password.clone(),
            ))
            .build();
        mailer.send(&msg)?;
        Ok(())
    }
}
