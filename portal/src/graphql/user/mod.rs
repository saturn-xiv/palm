pub mod email;

use chrono::Duration;
use juniper::GraphQLObject;

use super::super::{
    Jwt, Result,
    cache::redis::StandaloneConnection as Cache,
    models::user::{
        Dao as UserDao, Item as UserItem, Type as UserType, email::Item as EmailUserItem,
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
}
impl Layout {
    pub async fn new<R: Rbac>(rbac: &R, user: &UserItem) -> Result<Self> {
        user.is_enable()?;

        Ok(Self {
            lang: user.lang.clone(),
            timezone: user.timezone.clone(),
            name: user.name.clone(),
            avatar: user.avatar.clone(),
            is_administrator: rbac.is_administrator(user.id).await.is_ok(),
            roles: rbac.roles(user.id).await?,
            permissions: rbac.permissions(user.id).await?,
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
            user: Layout::new(rbac, &current_user.item).await?,
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
            token: jwt
                .sign(
                    CurrentUser::ISSUER,
                    subject,
                    vec![CurrentUser::SIGN_IN_AUDIENCE],
                    Duration::weeks(1),
                    Some(TokenPayload { r#type: type_ }),
                )
                .await?,
            user: Layout::new(rbac, &user).await?,
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
