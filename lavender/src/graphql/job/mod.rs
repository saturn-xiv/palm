pub mod git;

use std::any::type_name;

use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use portal::{
    Error, HttpError, Jwt, Result,
    cache::redis::StandaloneConnection as Cache,
    graphql::Session,
    models::{
        log::{Dao as LogDao, Level},
        user::Type as UserType,
    },
    orm::postgresql::Connection as Db,
    queue::rabbitmq::{BasicPublishOptions, Client as RabbitMq, FlexBuffersMessageSender},
    rbac::Rbac,
};
use serde::{Deserialize, Serialize};

use super::super::{
    Config,
    models::{job::Item as Job, task::Dao as TaskDao},
};
use super::{Plugin, ROLE as OPERATOR};

pub async fn launch<R: Rbac, J: Jwt, A: Into<String> + Clone>(
    ss: &Session,
    (db, cache, queue): (&mut Db, &mut Cache, &RabbitMq),
    (rbac, jwt): (&R, &J),
    config: &Config,
    (name, args): (&str, Vec<A>),
) -> Result<()> {
    let ip = ss.client_ip();
    let current_user = ss.current_user(db, cache, jwt).await?;
    rbac.has_role(current_user.id(), OPERATOR).await?;

    if current_user.type_ != UserType::Email {
        return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
    }

    let job = {
        let it = Job::new(&config.jobs_dir, name)?;
        it.validate(args.clone())?;
        it
    };

    let uid = db.transaction::<_, Error, _>(|tx| {
        let it = TaskDao::create(tx, &current_user.subject, &job, args.clone())?;
        LogDao::create::<Plugin, _>(
            tx,
            current_user.id(),
            Level::Info,
            ip,
            format!("Run job {}.", name),
        )?;
        Ok(it)
    })?;

    let task = Task {
        uid,
        name: name.to_string(),
        email: current_user.subject,
        args: args.into_iter().map(|x| x.into()).collect(),
    };

    FlexBuffersMessageSender::publish(
        queue,
        "",
        type_name::<Task>(),
        &task,
        BasicPublishOptions::default(),
    )
    .await?;

    Ok(())
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderJob")]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub inputs: Vec<Input>,
}

impl Item {
    pub async fn index<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        _config: &Config,
    ) -> Result<Vec<Self>> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        rbac.has_role(current_user.id(), OPERATOR).await?;
        // TODO
        todo!()
    }
    pub async fn by_id<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        _config: &Config,
        _id: &str,
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        rbac.has_role(current_user.id(), OPERATOR).await?;
        // TODO
        todo!()
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderJobInput")]
pub struct Input {
    pub id: String,
    pub label: String,
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderJobSelect")]
pub struct Select {
    pub id: String,
    pub options: Vec<SelectOption>,
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderJobSelectOption")]
pub struct SelectOption {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub args: Vec<String>,
}
