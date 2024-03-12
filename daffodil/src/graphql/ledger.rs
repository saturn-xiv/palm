use camelia::{
    models::{
        log::{Dao as LogDao, Level as LogLevel},
        user::Item as User,
    },
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};
use casbin::Enforcer;
use chrono::{Duration, NaiveDateTime};
use diesel::Connection as DieselConntection;
use hyper::StatusCode;
use juniper::{GraphQLEnum, GraphQLObject};
use log::{debug, warn};
use palm::{
    cache::redis::ClusterConnection as Cache, jwt::Jwt, rbac::Operation, session::Session, Error,
    HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::ledger::{Dao as LedgerDao, Item as Ledger},
    NAME,
};

#[derive(GraphQLObject)]
#[graphql(name = "DaffodilLedgerIndexResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub name: String,
    pub summary: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Ledger> for IndexResponseItem {
    fn from(x: Ledger) -> Self {
        Self {
            id: x.id,
            name: x.name.clone(),
            summary: x.summary.clone(),
            deleted_at: x.deleted_at,
            updated_at: x.updated_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "DaffodilLedgerIndexResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
}

impl IndexResponse {
    pub fn new<J: Jwt>(ss: &Session, db: &mut Db, ch: &mut Cache, jwt: &J) -> Result<Self> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let items = LedgerDao::by_user(db, user.id)?
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(Self { items })
    }
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 511))]
    pub summary: String,
}

impl Form {
    pub fn create<J: Jwt>(&self, ss: &Session, db: &mut Db, ch: &mut Cache, jwt: &J) -> Result<()> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        debug!("create ledger {}({})", self.name, self.summary);
        db.transaction::<_, Error, _>(move |db| {
            LedgerDao::create(db, user.id, &self.name, &self.summary)?;
            let it = LedgerDao::by_user_and_name(db, user.id, &self.name)?;
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
    ) -> Result<()> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let it = LedgerDao::by_id(db, id)?;
        if it.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)));
        }
        it.can_edit(enf, &user).await?;
        debug!(
            "update ledger {}({}) => {}({})",
            it.name, it.summary, self.name, self.summary
        );
        db.transaction::<_, Error, _>(move |db| {
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
) -> Result<()> {
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

#[derive(GraphQLEnum)]
#[graphql(name = "DaffodilLedgerExportType")]
pub enum ExportType {
    Pdf,
    Html,
}
#[derive(Validate)]
pub struct ExportRequest {
    #[validate(url, length(min = 1, max = 127))]
    pub home: String,
    pub r#type: ExportType,
    pub id: i32,
    #[validate(range(min = 60))]
    pub ttl: i64,
}
impl ExportRequest {
    pub const AUDIENCE: &'static str = "daffodil.ledgers.export";
    pub async fn handle<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<String> {
        self.validate()?;
        let ttl = Duration::try_seconds(self.ttl).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("bad ttl".to_string()),
        )))?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let it = LedgerDao::by_id(db, self.id)?;
        it.can_show(enf, &user).await?;
        let token = jwt.sign(NAME, &it.uid, Self::AUDIENCE, ttl)?;

        Ok(format!("{}/daffodil/ledgers/{}", self.home, token))
    }
}

impl Ledger {
    pub async fn can_show(&self, enf: &Mutex<Enforcer>, user: &User) -> Result<()> {
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

    pub async fn can_edit(&self, enf: &Mutex<Enforcer>, user: &User) -> Result<()> {
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
