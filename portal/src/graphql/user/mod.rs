pub mod email;

use chrono::Duration;
use juniper::GraphQLObject;

use super::super::{
    Jwt, Result,
    models::user::{Dao as UserDao, Type as UserType, email::Item as EmailUserItem},
    orm::postgresql::Connection as Db,
    rbac::Permission,
    rbac::Rbac,
};
use super::{CurrentUser, Pagination, TokenPayload};

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
#[graphql(name = "UserSignInResponse")]
pub struct SignInResponse {
    pub token: String,
    pub lang: String,
    pub timezone: String,
    pub name: String,
    pub avatar: Option<String>,
    pub is_administrator: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<Permission>,
}

impl SignInResponse {
    pub async fn new<R: Rbac, J: Jwt>(
        db: &mut Db,
        rbac: &R,
        jwt: &J,
        user: i64,
        type_: UserType,
        name: &str,
        avatar: Option<&str>,
    ) -> Result<Self> {
        let user = UserDao::by_id(db, user)?;
        user.is_enable()?;

        Ok(Self {
            token: jwt
                .sign(
                    CurrentUser::ISSUER,
                    &user.uid.clone(),
                    vec![CurrentUser::SIGN_IN_AUDIENCE],
                    Duration::days(7),
                    Some(TokenPayload { r#type: type_ }),
                )
                .await?,
            lang: user.lang.clone(),
            timezone: user.timezone.clone(),
            name: name.to_string(),
            avatar: match avatar {
                Some(it) => Some(it.to_string()),
                None => user.avatar.clone(),
            },
            is_administrator: rbac.is_administrator(user.id).await.is_ok(),
            roles: rbac.roles(user.id).await?,
            permissions: rbac.permissions(user.id).await?,
        })
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "IndexUserResponse")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}
