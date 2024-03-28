use camelia::{
    graphql::attachment::Show as ShowAttachment,
    models::{
        attachment::Dao as AttachmentDao,
        log::{Dao as LogDao, Level as LogLevel},
        user::{Details as UserDetails, Item as User},
    },
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};
use casbin::Enforcer;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::Connection as DieselConntection;
use hyper::StatusCode;
use juniper::{GraphQLEnum, GraphQLObject};
use log::{debug, info, warn};
use palm::{
    cache::redis::ClusterConnection as Cache, duration_from_days, jwt::Jwt, minio::Client as Minio,
    rbac::Operation, session::Session, Error, HttpError,
};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::ledger::{Dao as LedgerDao, Item as Ledger},
    NAME,
};

pub async fn share<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
    (begin, end): (&str, &str),
) -> Result<String, Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;

    let it = LedgerDao::by_id(db, id)?;
    it.can_show(enf, &user).await?;

    let format = "%Y-%m-%d";

    let token = jwt.sign_by_range(
        NAME,
        &it.uid,
        Ledger::SHOW,
        NaiveDate::parse_from_str(begin, format)?.and_time(NaiveTime::MIN),
        NaiveDate::parse_from_str(end, format)?
            .and_hms_opt(23, 59, 59)
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("bad time".to_string()),
            )))?,
    )?;
    debug!(
        "generate {token} for share ledger ({}, {}) ({begin},{end}) by {}",
        it.id, it.name, user
    );
    Ok(format!("/daffodil/ledgers/{token}"))
}

#[derive(GraphQLObject)]
#[graphql(name = "DaffodilIndexLedgerResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub owner: UserDetails,
    pub cover: ShowAttachment,
    pub name: String,
    pub summary: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl IndexResponseItem {
    async fn new(db: &mut Db, s3: &Minio, x: &Ledger) -> Result<Self, Error> {
        let cover = AttachmentDao::by_resource::<Ledger>(db, x.id)?
            .into_iter()
            .nth(0)
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("empty cover".to_string()),
            )))?;

        let it = Self {
            id: x.id,
            owner: UserDetails::new(db, x.owner_id)?,
            cover: ShowAttachment::new(s3, &cover, duration_from_days(1)?).await?,
            name: x.name.clone(),
            summary: x.summary.clone(),
            deleted_at: x.deleted_at,
            updated_at: x.updated_at,
        };
        Ok(it)
    }

    pub async fn show<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        s3: &Minio,
        id: i32,
    ) -> Result<Self, Error> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let it = LedgerDao::by_id(db, id)?;
        it.can_show(enf, &user).await?;

        Self::new(db, s3, &it).await
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "DaffodilIndexLedgerResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
}

impl IndexResponse {
    pub async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &J,
        s3: &Minio,
    ) -> Result<Self, Error> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        let mut items = Vec::new();
        for it in LedgerDao::by_user(db, user.id)?.iter() {
            items.push(IndexResponseItem::new(db, s3, it).await?);
        }

        Ok(Self { items })
    }
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 511))]
    pub summary: String,
    pub cover: i32,
}

impl Form {
    pub fn create<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &J,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        debug!("create ledger {}({})", self.name, self.summary);
        db.transaction::<_, Error, _>(move |db| {
            AttachmentDao::by_id(db, self.cover)?;
            LedgerDao::create(db, user.id, &self.name, &self.summary)?;
            let it = LedgerDao::by_user_and_name(db, user.id, &self.name)?;
            AttachmentDao::associate::<Ledger>(db, self.cover, it.id)?;
            LogDao::add::<_, Ledger>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(it.id),
                &format!("create ledger {}", self.name),
            )?;
            Ok(())
        })?;
        Ok(())
    }

    pub async fn update<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        id: i32,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let it = LedgerDao::by_id(db, id)?;
        it.can_edit(enf, &user).await?;
        debug!(
            "update ledger {}({}) => {}({})",
            it.name, it.summary, self.name, self.summary
        );
        db.transaction::<_, Error, _>(move |db| {
            AttachmentDao::by_id(db, self.cover)?;
            AttachmentDao::clear::<Ledger>(db, it.id)?;
            AttachmentDao::associate::<Ledger>(db, self.cover, id)?;
            LedgerDao::update(db, id, &self.name, &self.summary)?;
            LogDao::add::<_, Ledger>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(it.id),
                &format!("update ledger {}", self.name),
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

pub async fn enable<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
    on: bool,
) -> Result<(), Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;

    let it = LedgerDao::by_id(db, id)?;
    it.can_edit(enf, &user).await?;
    warn!(
        "{} {}({})",
        if on { "enable" } else { "disable" },
        it.name,
        it.summary
    );
    db.transaction::<_, Error, _>(move |db| {
        LedgerDao::enable(db, id, on)?;
        LogDao::add::<_, Ledger>(
            db,
            user.id,
            NAME,
            LogLevel::Info,
            &ss.client_ip,
            Some(it.id),
            &format!("{} ledger ", if on { "enable" } else { "disable" }),
        )?;
        Ok(())
    })?;
    Ok(())
}

#[derive(
    GraphQLEnum, EnumString, EnumDisplay, Serialize, Deserialize, PartialEq, Eq, Debug, Clone,
)]
#[graphql(name = "DaffodilLedgerExportFormat")]
pub enum ExportFormat {
    Pdf,
    Html,
}
#[derive(Validate)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub id: i32,
}
impl ExportRequest {
    pub async fn handle<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let it = LedgerDao::by_id(db, self.id)?;
        it.can_show(enf, &user).await?;
        // TODO send email
        info!(
            "export ledger({},{},{}) by {user}",
            it.id, it.name, self.format
        );

        Ok(())
    }
}

impl Ledger {
    pub async fn can_show(&self, enf: &Mutex<Enforcer>, user: &User) -> Result<(), Error> {
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)));
        }
        if self.owner_id == user.id {
            return Ok(());
        }
        if user.is_administrator(enf).await.is_ok() {
            return Ok(());
        }
        if user
            .can::<Self, _>(enf, Operation::Show, Some(self.id))
            .await
            .is_ok()
        {
            return Ok(());
        }
        if user
            .can::<Self, _>(enf, Operation::Edit, Some(self.id))
            .await
            .is_ok()
        {
            return Ok(());
        }
        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }

    pub async fn can_edit(&self, enf: &Mutex<Enforcer>, user: &User) -> Result<(), Error> {
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)));
        }
        if self.owner_id == user.id {
            return Ok(());
        }
        if user.is_administrator(enf).await.is_ok() {
            return Ok(());
        }
        if user
            .can::<Self, _>(enf, Operation::Edit, Some(self.id))
            .await
            .is_ok()
        {
            return Ok(());
        }

        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }
}
