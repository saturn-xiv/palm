use camelia::{i18n::I18n, orm::postgresql::Connection as Db, services::CurrentUserAdapter};
use casbin::Enforcer;
use juniper::GraphQLObject;
use palm::{cache::redis::ClusterConnection as Cache, jwt::Jwt, session::Session, Error};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(name = "Route")]
pub struct Route {
    pub path: String,
    pub name: String,
    pub routes: Vec<Route>,
}

impl Route {
    pub async fn load<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<Vec<Self>, Error> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let mut items = Vec::new();
        if user.is_administrator(enf).await.is_ok() {
            items.push(Self {
                path: "/dashboard/settings".to_string(),
                name: I18n::t(db, &ss.lang, "settings.index.title", &None::<String>),
                routes: vec![
                    Self {
                        path: "/dashboard/settings/site/status".to_string(),
                        name: I18n::t(db, &ss.lang, "settings.site.status.title", &None::<String>),
                        routes: Vec::new(),
                    },
                    Self {
                        path: "/dashboard/settings/site/seo".to_string(),
                        name: I18n::t(db, &ss.lang, "settings.site.seo.title", &None::<String>),
                        routes: Vec::new(),
                    },
                    Self {
                        path: "/dashboard/settings/site/info".to_string(),
                        name: I18n::t(db, &ss.lang, "settings.site.info.title", &None::<String>),
                        routes: Vec::new(),
                    },
                    Self {
                        path: "/dashboard/settings/locales".to_string(),
                        name: I18n::t(
                            db,
                            &ss.lang,
                            "settings.locales.index.title",
                            &None::<String>,
                        ),
                        routes: Vec::new(),
                    },
                ],
            })
        }
        items.push(Self {
            path: "/dashboard/personal".to_string(),
            name: I18n::t(db, &ss.lang, "personal.index.title", &None::<String>),
            routes: vec![
                Self {
                    path: "/dashboard/personal/logs".to_string(),
                    name: I18n::t(db, &ss.lang, "personal.logs.title", &None::<String>),
                    routes: Vec::new(),
                },
                Self {
                    path: "/dashboard/personal/profile".to_string(),
                    name: I18n::t(db, &ss.lang, "personal.profile.title", &None::<String>),
                    routes: Vec::new(),
                },
                Self {
                    path: "/dashboard/personal/attachments".to_string(),
                    name: I18n::t(db, &ss.lang, "personal.attachments.title", &None::<String>),
                    routes: Vec::new(),
                },
            ],
        });

        Ok(items)
    }
}
