use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use diesel::Connection as DieselConntection;
use hibiscus::cache::redis::{ClusterConnection as Cache, Pool as CachePool};
use palm::{
    azalea::v1::{IdRequest, Pager, Pagination},
    camelia::v1,
    gourd::Policy,
    to_timestamp, try_grpc, Error, GrpcResult, Jwt, Result, Thrift,
};
use tonic::{Request, Response};
use validator::Validate;

use super::super::{
    models::locale::{Dao as LocaleDao, Item as Locale},
    orm::postgresql::{Connection as Db, Pool as DbPool},
};
use super::{CurrentUserAdapter, Session};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
}

#[tonic::async_trait]
impl v1::locale_server::Locale for Service {
    async fn set(&self, req: Request<v1::LocaleSetRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();

        let form = {
            let req = req.into_inner();
            Form {
                lang: req.lang.clone(),
                code: req.code.clone(),
                message: req.message,
            }
        };
        try_grpc!(form.save(&ss, db, ch, jwt, policy))?;
        Ok(Response::new(()))
    }
    async fn destroy(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(LocaleDao::delete(db, req.id))?;
        Ok(Response::new(()))
    }
    async fn by_lang(
        &self,
        req: Request<v1::LocaleByLangRequest>,
    ) -> GrpcResult<v1::LocaleByLangResponse> {
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let req = req.into_inner();
        let items = try_grpc!(LocaleDao::by_lang(db, &req.lang))?
            .into_iter()
            .map(|x| x.into())
            .collect();
        Ok(Response::new(v1::LocaleByLangResponse { items }))
    }
    async fn index(&self, req: Request<Pager>) -> GrpcResult<v1::LocaleIndexResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let pager = req.into_inner();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }
        let total = try_grpc!(LocaleDao::count(db))?;
        let items = try_grpc!(LocaleDao::all(db, pager.offset(total), pager.size()))?
            .into_iter()
            .map(|x| x.into())
            .collect();
        Ok(Response::new(v1::LocaleIndexResponse {
            items,
            pagination: Some(Pagination::new(&pager, total)),
        }))
    }
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 7))]
    pub lang: String,
    #[validate(length(min = 1, max = 255))]
    pub code: String,
    #[validate(length(min = 1))]
    pub message: String,
}

impl Form {
    pub fn save<J: Jwt, P: Policy>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &J,
        policy: &P,
    ) -> Result<()> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(policy)?;

        db.transaction::<_, Error, _>(move |db| {
            match LocaleDao::by_lang_and_code(db, &self.lang, &self.code) {
                Ok(ref it) => LocaleDao::update(db, it.id, &self.message)?,
                Err(_) => LocaleDao::create(db, &self.lang, &self.code, &self.message)?,
            };
            Ok(())
        })?;

        Ok(())
    }
}

impl From<Locale> for v1::locale_index_response::Item {
    fn from(x: Locale) -> Self {
        Self {
            id: x.id,
            lang: x.lang.clone(),
            code: x.code.clone(),
            message: x.message.clone(),
            updated_at: Some(to_timestamp!(x.updated_at)),
        }
    }
}
