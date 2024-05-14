use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use chrono::Utc;
use diesel::{prelude::*, Connection as DieselConntection};
use hibiscus::cache::redis::{nodes as cluster_nodes, Pool as CachePool};
use palm::{camelia::v1, to_timestamp, try_grpc, Error, GrpcResult, Result, Thrift, GIT_VERSION};
use tonic::{Request, Response};
use validator::Validate;

use super::super::{
    i18n::I18n,
    models::{locale::Dao as LocaleDao, setting::Protobuf as ProtobufProtocol},
    orm::{
        postgresql::{schema::schema_migrations, Connection as Db, Pool as DbPool},
        Dao as VersionDao,
    },
};
use super::{CurrentUserAdapter, Session};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
}

#[tonic::async_trait]
impl v1::site_server::Site for Service {
    async fn layout(&self, req: Request<()>) -> GrpcResult<v1::SiteLayoutResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let aes = self.loquat.deref();

        Ok(Response::new(v1::SiteLayoutResponse {
            lang: ss.lang.clone(),
            languages: try_grpc!(LocaleDao::languages(db))?,
            title: I18n::t(db, &ss.lang, v1::SiteSetInfoRequest::TITLE, &None::<String>),
            subhead: I18n::t(
                db,
                &ss.lang,
                v1::SiteSetInfoRequest::SUBHEAD,
                &None::<String>,
            ),
            description: I18n::t(
                db,
                &ss.lang,
                v1::SiteSetInfoRequest::DESCRIPTION,
                &None::<String>,
            ),
            copyright: I18n::t(
                db,
                &ss.lang,
                v1::SiteSetInfoRequest::COPYRIGHT,
                &None::<String>,
            ),
            author: Some(
                ProtobufProtocol::get::<v1::site_layout_response::Author, _>(db, aes, None)
                    .unwrap_or_default(),
            ),
            keywords: ProtobufProtocol::get::<v1::SiteSetKeywordsRequest, _>(db, aes, None)
                .unwrap_or_default()
                .items,
            favicon: ProtobufProtocol::get::<v1::SiteSetFaviconRequest, _>(db, aes, None)
                .unwrap_or_default()
                .url,
            gab: ProtobufProtocol::get::<v1::site_layout_response::GabCode, _>(db, aes, None).ok(),
            icp: ProtobufProtocol::get::<v1::site_layout_response::IcpCode, _>(db, aes, None).ok(),
            version: GIT_VERSION.to_string(),
            created_at: Some(to_timestamp!(Utc::now().naive_local())),
        }))
    }

    async fn status(&self, req: Request<()>) -> GrpcResult<v1::SiteStatusResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        Ok(Response::new(v1::SiteStatusResponse {
            postgresql: Some(try_grpc!(new_postgresql_status(db))?),
            redis: Some(v1::site_status_response::Redis {
                nodes: try_grpc!(cluster_nodes(ch))?,
            }),
            opensearch: Some(v1::site_status_response::OpenSearch::default()),
        }))
    }
    async fn set_info(&self, req: Request<v1::SiteSetInfoRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let form = InfoForm {
                title: &req.title,
                subhead: &req.subhead,
                description: &req.description,
                copyright: &req.copyright,
            };
            try_grpc!(form.validate())?;
        }

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            I18n::set(db, &req.lang, v1::SiteSetInfoRequest::TITLE, &req.title)?;
            I18n::set(db, &req.lang, v1::SiteSetInfoRequest::SUBHEAD, &req.subhead)?;
            I18n::set(
                db,
                &req.lang,
                v1::SiteSetInfoRequest::DESCRIPTION,
                &req.description,
            )?;
            I18n::set(
                db,
                &req.lang,
                v1::SiteSetInfoRequest::COPYRIGHT,
                &req.copyright,
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn set_keywords(&self, req: Request<v1::SiteSetKeywordsRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(ProtobufProtocol::set::<v1::SiteSetKeywordsRequest, _>(
            db, jwt_aes, None, &req, false
        ))?;
        Ok(Response::new(()))
    }
    async fn set_author(&self, req: Request<v1::site_layout_response::Author>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let form = AuthorForm {
                name: &req.name,
                email: &req.email,
            };
            try_grpc!(form.validate())?;
        }
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(
            ProtobufProtocol::set::<v1::site_layout_response::Author, _>(
                db, jwt_aes, None, &req, false
            )
        )?;
        Ok(Response::new(()))
    }
    async fn set_favicon(&self, req: Request<v1::SiteSetFaviconRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        {
            let form = FaviconForm { url: &req.url };
            try_grpc!(form.validate())?;
        }
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(ProtobufProtocol::set::<v1::SiteSetFaviconRequest, _>(
            db, jwt_aes, None, &req, false
        ))?;
        Ok(Response::new(()))
    }

    async fn set_icp_code(
        &self,
        req: Request<v1::site_layout_response::IcpCode>,
    ) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(
            ProtobufProtocol::set::<v1::site_layout_response::IcpCode, _>(
                db, jwt_aes, None, &req, true
            )
        )?;
        Ok(Response::new(()))
    }
    async fn get_icp_code(
        &self,
        req: Request<()>,
    ) -> GrpcResult<v1::site_layout_response::IcpCode> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let it = try_grpc!(
            ProtobufProtocol::get::<v1::site_layout_response::IcpCode, _>(db, jwt_aes, None)
        )?;
        Ok(Response::new(it))
    }
    async fn set_gab_code(
        &self,
        req: Request<v1::site_layout_response::GabCode>,
    ) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(
            ProtobufProtocol::set::<v1::site_layout_response::GabCode, _>(
                db, jwt_aes, None, &req, true
            )
        )?;
        Ok(Response::new(()))
    }
    async fn get_gab_code(
        &self,
        req: Request<()>,
    ) -> GrpcResult<v1::site_layout_response::GabCode> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let it = try_grpc!(
            ProtobufProtocol::get::<v1::site_layout_response::GabCode, _>(db, jwt_aes, None)
        )?;
        Ok(Response::new(it))
    }

    async fn set_baidu(&self, req: Request<v1::BaiduProfile>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(ProtobufProtocol::set::<v1::BaiduProfile, _>(
            db, jwt_aes, None, &req, true
        ))?;
        Ok(Response::new(()))
    }
    async fn get_baidu(&self, req: Request<()>) -> GrpcResult<v1::BaiduProfile> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let it = try_grpc!(ProtobufProtocol::get::<v1::BaiduProfile, _>(
            db, jwt_aes, None
        ))?;
        Ok(Response::new(it))
    }
    async fn set_index_now(&self, req: Request<v1::IndexNowProfile>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(ProtobufProtocol::set::<v1::IndexNowProfile, _>(
            db, jwt_aes, None, &req, true
        ))?;
        Ok(Response::new(()))
    }
    async fn get_index_now(&self, req: Request<()>) -> GrpcResult<v1::IndexNowProfile> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let it = try_grpc!(ProtobufProtocol::get::<v1::IndexNowProfile, _>(
            db, jwt_aes, None
        ))?;
        Ok(Response::new(it))
    }
    async fn set_google(&self, req: Request<v1::GoogleProfile>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(ProtobufProtocol::set::<v1::GoogleProfile, _>(
            db, jwt_aes, None, &req, true
        ))?;
        Ok(Response::new(()))
    }
    async fn get_google(&self, req: Request<()>) -> GrpcResult<v1::GoogleProfile> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let it = try_grpc!(ProtobufProtocol::get::<v1::GoogleProfile, _>(
            db, jwt_aes, None
        ))?;
        Ok(Response::new(it))
    }
    async fn set_re_captcha(&self, req: Request<v1::ReCaptchaProfile>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(ProtobufProtocol::set::<v1::ReCaptchaProfile, _>(
            db, jwt_aes, None, &req, true
        ))?;
        Ok(Response::new(()))
    }
    async fn get_re_captcha(&self, req: Request<()>) -> GrpcResult<v1::ReCaptchaProfile> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt_aes = self.loquat.deref();
        let policy = self.gourd.deref();

        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt_aes))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let it = try_grpc!(ProtobufProtocol::get::<v1::ReCaptchaProfile, _>(
            db, jwt_aes, None
        ))?;
        Ok(Response::new(it))
    }
}

#[derive(Validate)]
struct FaviconForm<'a> {
    #[validate(length(min = 1, max = 127))]
    url: &'a str,
}

#[derive(Validate)]
struct InfoForm<'a> {
    #[validate(length(min = 1, max = 63))]
    title: &'a str,
    #[validate(length(min = 1, max = 31))]
    subhead: &'a str,
    #[validate(length(min = 1, max = 511))]
    description: &'a str,
    #[validate(length(min = 1, max = 63))]
    copyright: &'a str,
}

#[derive(Validate)]
struct AuthorForm<'a> {
    #[validate(length(min = 1, max = 32))]
    name: &'a str,
    #[validate(email, length(min = 1, max = 63))]
    email: &'a str,
}

fn new_postgresql_status(db: &mut Db) -> Result<v1::site_status_response::PostgreSql> {
    let migrations = schema_migrations::dsl::schema_migrations
        .select(schema_migrations::dsl::version)
        .load(db)?;
    let version = VersionDao::version(db)?;
    Ok(v1::site_status_response::PostgreSql {
        migrations,
        version,
    })
}
