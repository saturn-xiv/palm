use lettre::{
    message::{header::ContentType, Attachment, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use mime::Mime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::Result;

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
    pub fn send(
        &self,
        to: &Address,
        cc: &[Address],
        bcc: &[Address],
        subject: &str,
        body: (bool, String),
        attachments: Vec<(String, Mime, Vec<u8>, bool)>,
    ) -> Result<()> {
        let account = self.from.mailbox()?;
        let to = to.mailbox()?;

        let msg = {
            let mut builder = Message::builder()
                .from(account.clone())
                .reply_to(account)
                .to(to)
                .subject(subject);
            // cc
            {
                for it in cc.iter() {
                    builder = builder.cc(it.mailbox()?);
                }
                for it in cc.iter() {
                    builder = builder.cc(it.mailbox()?);
                }
            }
            // bcc
            {
                for it in bcc.iter() {
                    builder = builder.bcc(it.mailbox()?);
                }
                for it in bcc.iter() {
                    builder = builder.bcc(it.mailbox()?);
                }
            }
            // attachments
            let mut part = MultiPart::alternative().build();
            {
                if body.0 {
                    part = part.singlepart(SinglePart::html(body.1.clone()));
                } else {
                    part = part.singlepart(SinglePart::plain(body.1.clone()));
                }
                for (name, content_type, content, inline) in attachments.iter() {
                    let content_type: ContentType = content_type.clone().into();
                    part = if *inline {
                        part.singlepart(
                            Attachment::new_inline(name.clone())
                                .body(content.to_vec(), content_type),
                        )
                    } else {
                        part.singlepart(
                            Attachment::new(name.clone()).body(content.to_vec(), content_type),
                        )
                    };
                }
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
