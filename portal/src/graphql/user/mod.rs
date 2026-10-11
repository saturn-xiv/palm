pub mod email;
pub mod google_oauth2;
pub mod wechat_mini_program;
pub mod wechat_oauth2;

use std::any::type_name;

use chrono::Duration;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::super::{
    Jwt, Result,
    cache::{FlexBuffersCacher, redis::StandaloneConnection as Cache},
    models::user::{
        Dao as UserDao, Item as UserItem, Type as UserType,
        email::{Dao as EmailUserDao, Item as EmailUserItem},
        google_oauth2::Dao as GoogleOauth2UserDao,
        wechat_mini_program::Dao as WechatMiniProgramUserDao,
        wechat_oauth2::Dao as WechatOauth2UserDao,
    },
    orm::postgresql::Connection as Db,
    rbac::{Permission, Rbac},
};
use super::{CurrentUser, Pagination, Session, TokenPayload, site::Layout as SiteLayout};

#[derive(Debug, GraphQLObject)]
#[graphql(name = "User")]
pub struct Item {
    pub uid: String,
    pub label: String,
    pub code: String,
    pub r#type: UserType,
}

impl Item {
    pub fn by_email(db: &mut Db, it: &EmailUserItem) -> Result<Self> {
        let user = UserDao::by_id(db, it.user_id)?;
        Ok(Self {
            uid: user.uid,
            label: it.to_string(),
            code: it.email.clone(),
            r#type: UserType::Email,
        })
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "UserLayout")]
pub struct Layout {
    pub lang: String,
    pub timezone: String,
    pub name: String,
    pub avatar: Option<String>,
    pub is_administrator: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<Permission>,
    pub has_email: bool,
    pub has_wechat_oauth2: bool,
    pub has_wechat_mini_program: bool,
    pub has_google_oauth2: bool,
}
impl Layout {
    pub async fn new<R: Rbac>(db: &mut Db, rbac: &R, user: &UserItem) -> Result<Self> {
        user.is_enable()?;

        Ok(Self {
            lang: user.lang.clone(),
            timezone: user.timezone.clone(),
            name: user.name.clone(),
            avatar: user.avatar.clone(),
            is_administrator: rbac.is_administrator(user.id).await.is_ok(),
            roles: rbac.roles(user.id).await?,
            permissions: rbac.permissions(user.id).await?,
            has_email: !EmailUserDao::by_user(db, user.id)?.is_empty(),
            has_wechat_oauth2: !WechatOauth2UserDao::by_user(db, user.id)?.is_empty(),
            has_wechat_mini_program: !WechatMiniProgramUserDao::by_user(db, user.id)?.is_empty(),
            has_google_oauth2: !GoogleOauth2UserDao::by_user(db, user.id)?.is_empty(),
        })
    }
}
#[derive(Debug, GraphQLObject)]
#[graphql(name = "RefreshResponse")]
pub struct RefreshResponse {
    pub user: Layout,
    pub site: SiteLayout,
}

impl RefreshResponse {
    pub async fn new<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        jwt: &J,
        rbac: &R,
        version: &str,
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        let lang = current_user.lang()?;
        Ok(Self {
            user: Layout::new(db, rbac, &current_user.item).await?,
            site: SiteLayout::new(db, cache, &lang, version)?,
        })
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "UserSignInResponse")]
pub struct SignInResponse {
    pub token: String,
    pub user: Layout,
    pub site: SiteLayout,
}

impl SignInResponse {
    pub async fn new<R: Rbac, J: Jwt>(
        db: &mut Db,
        cache: &mut Cache,
        (rbac, jwt): (&R, &J),
        (user, type_, subject): (i64, UserType, &str),
        version: &str,
    ) -> Result<Self> {
        let user = UserDao::by_id(db, user)?;
        let lang = user.lang.parse()?;
        Ok(Self {
            token: CurrentUser::token(
                jwt,
                type_,
                subject,
                vec![CurrentUser::SIGN_IN_AUDIENCE],
                Duration::weeks(1),
            )
            .await?,
            user: Layout::new(db, rbac, &user).await?,
            site: SiteLayout::new(db, cache, &lang, version)?,
        })
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "IndexUserResponse")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub user: Option<String>,
}

impl State {
    pub fn new(cache: &mut Cache, key: &str) -> Result<Self> {
        cache.get(Self::key(key))
    }
    pub fn save(&self, cache: &mut Cache, key: &str, ttl: Duration) -> Result<()> {
        cache.set(Self::key(key), self, Some(ttl))?;
        Ok(())
    }

    fn key(s: &str) -> String {
        format!("{}.{}", type_name::<Self>(), s)
    }
}
