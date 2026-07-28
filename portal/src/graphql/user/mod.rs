pub mod email;

use juniper::GraphQLObject;

use super::super::{
    Result,
    models::user::{Dao as UserDao, Type as UserType, email::Item as EmailUserItem},
    orm::postgresql::Connection as Db,
};
use super::Pagination;

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
#[graphql(name = "IndexUserResponse")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}
