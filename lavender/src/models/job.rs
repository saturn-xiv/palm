use std::any::type_name;
use std::path::Path;
use std::{collections::BTreeMap, fs::read_dir};

use flatbuffers::FlatBufferBuilder;
use hyacinth::email_v1::{
    Address as EmailAddress, AddressArgs as EmailAddressArgs, Body as EmailBody,
    BodyArgs as EmailBodyArgs, Task as EmailTask, TaskArgs as EmailTaskArgs,
};
use hyper::StatusCode;
use portal::{
    HttpError, Result,
    content_types::APPLICATION_X_FLATBUFFERS,
    parse_toml,
    queue::rabbitmq::{BasicPublishOptions, Client as RabbitMq},
    shell,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub version: String,
    pub command: String,
    pub description: String,
    pub args: Vec<Arg>,
}

impl Item {
    const CONFIG_FILE: &str = "config.toml";

    pub fn execute<P: AsRef<Path>, A: Into<String>>(
        &self,
        working_dir: P,
        args: Vec<A>,
    ) -> Result<String> {
        if args.len() != self.args.len() {
            return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
        }
        shell(working_dir, &self.command, args)
    }

    pub async fn report<A: Into<String>>(
        &self,
        queue: &RabbitMq,
        to: &str,
        bcc: Vec<A>,
        body: &str,
        succeed: bool,
    ) -> Result<()> {
        let mut builder = FlatBufferBuilder::new();
        {
            let subject = builder.create_string(&format!(
                "{}({}) {}",
                self.name,
                self.version,
                if succeed { "succeed" } else { "failed" }
            ));
            let body_content = builder.create_string(body);
            let to_email = builder.create_string(to);
            let mut bcc_offsets = Vec::new();
            for it in bcc.into_iter() {
                let email: String = it.into();
                let email = builder.create_string(&email);
                let it = EmailAddress::create(
                    &mut builder,
                    &EmailAddressArgs {
                        name: None,
                        email: Some(email),
                    },
                );
                bcc_offsets.push(it);
            }
            let bcc = builder.create_vector(&bcc_offsets);

            let body = EmailBody::create(
                &mut builder,
                &EmailBodyArgs {
                    html: true,
                    content: Some(body_content),
                },
            );

            let to = EmailAddress::create(
                &mut builder,
                &EmailAddressArgs {
                    name: None,
                    email: Some(to_email),
                },
            );

            let task = EmailTask::create(
                &mut builder,
                &EmailTaskArgs {
                    to: Some(to),
                    subject: Some(subject),
                    body: Some(body),
                    bcc: Some(bcc),
                    ..Default::default()
                },
            );

            builder.finish(task, None);
        }
        let task: &[u8] = builder.finished_data();
        queue
            .publish(
                "",
                type_name::<EmailTask>(),
                APPLICATION_X_FLATBUFFERS,
                task,
                BasicPublishOptions::default(),
            )
            .await?;
        Ok(())
    }

    pub fn new<P: AsRef<Path>>(root: P, id: &str) -> Result<Self> {
        parse_toml({
            let it = root.as_ref();
            it.join(id).join(Self::CONFIG_FILE)
        })
    }
    pub fn load<P: AsRef<Path>>(root: P) -> Result<BTreeMap<String, Self>> {
        let mut items = BTreeMap::new();
        for entry in read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir()
                && let Some(id) = path.file_name()
                && let Some(id) = id.to_str()
            {
                items.insert(id.to_string(), parse_toml(path.join(Self::CONFIG_FILE))?);
            }
        }
        Ok(items)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Arg {
    Text {
        id: String,
        label: String,
    },
    Select {
        id: String,
        label: String,
        options: Vec<String>,
    },
    Git {
        id: String,
        label: String,
        url: String,
        branch: String,
    },
}
