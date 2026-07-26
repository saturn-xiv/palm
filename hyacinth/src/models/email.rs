use std::io::Error as IoError;
use std::result::Result as StdResult;

use lettre::{
    Message,
    address::AddressError,
    error::Error as LettreError,
    message::{Attachment, Mailbox, MultiPart, SinglePart, header::ContentType},
};

use super::super::email_v1::{Address, Task};

impl<'a> TryFrom<Task<'a>> for Message {
    type Error = LettreError;

    fn try_from(task: Task<'a>) -> StdResult<Self, Self::Error> {
        let mut builder = Message::builder()
            .subject(task.subject())
            .reply_to(
                Mailbox::try_from(task.to())
                    .map_err(|e| LettreError::Io(IoError::other(e.to_string())))?,
            )
            .from(
                Mailbox::try_from(task.from())
                    .map_err(|e| LettreError::Io(IoError::other(e.to_string())))?,
            );

        if let Some(it) = task.reply_to() {
            builder = builder.to(Mailbox::try_from(it)
                .map_err(|e| LettreError::Io(IoError::other(e.to_string())))?);
        }
        if let Some(items) = task.cc() {
            for it in items.iter() {
                builder = builder.cc(Mailbox::try_from(it)
                    .map_err(|e| LettreError::Io(IoError::other(e.to_string())))?);
            }
        }
        if let Some(items) = task.bcc() {
            for it in items.iter() {
                builder = builder.bcc(
                    Mailbox::try_from(it)
                        .map_err(|e| LettreError::Io(IoError::other(e.to_string())))?,
                );
            }
        }

        let mut parts = {
            MultiPart::mixed().singlepart(if task.body().html() {
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(task.body().content().to_string())
            } else {
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(task.body().content().to_string())
            })
        };

        if let Some(items) = task.attachments() {
            for it in items.iter() {
                let part = match it.inline_id() {
                    // <img src="cid:123">
                    Some(content_id) => Attachment::new_inline_with_name(
                        content_id.to_string(),
                        it.name().to_string(),
                    )
                    .body(
                        it.body().bytes().to_vec(),
                        ContentType::parse(it.content_type())
                            .map_err(|e| LettreError::Io(IoError::other(e.to_string())))?,
                    ),
                    None => Attachment::new(it.name().to_string()).body(
                        it.body().bytes().to_vec(),
                        ContentType::parse(it.content_type())
                            .map_err(|e| LettreError::Io(IoError::other(e.to_string())))?,
                    ),
                };
                parts = parts.singlepart(part);
            }
        }

        builder.multipart(parts)
    }
}

impl<'a> TryFrom<Address<'a>> for Mailbox {
    type Error = AddressError;

    fn try_from(it: Address<'a>) -> StdResult<Self, Self::Error> {
        Ok(Self {
            name: Some(it.name().to_string()),
            email: it.email().parse()?,
        })
    }
}
