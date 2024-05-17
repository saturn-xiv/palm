pub mod account;
pub mod book;
pub mod merchant;
pub mod transaction;

use camelia::{
    models::{
        attachment::Dao as AttachmentDao,
        user::{email::Item as EmailUser, Dao as UserDao, Item as User},
    },
    orm::postgresql::Connection as Db,
};
use chrono::Duration;
use palm::{daffodil::v1, jasmine::S3, to_timestamp, Result};

use super::models::{account::Item as Account, book::Item as Book, merchant::Item as Merchant};

pub fn new_account_item<S: S3>(
    db: &mut Db,
    s3: &S,
    x: &Account,
) -> Result<v1::account_index_response::Item> {
    let cover = match x.cover_id {
        Some(id) => {
            let it = AttachmentDao::by_id(db, id)?;
            let url = it.url(s3, Duration::hours(1))?;
            Some(url)
        }
        None => None,
    };
    let user = UserDao::by_id(db, x.user_id)?;
    Ok(v1::account_index_response::Item {
        id: x.id,
        name: x.name.clone(),
        description: x.description.clone(),
        deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
        updated_at: Some(to_timestamp!(x.updated_at)),
        currency: x.currency.clone(),
        r#type: x.type_,
        user: Some(new_user_detail(&user)),
        cover,
    })
}

pub fn new_book_item<S: S3>(
    db: &mut Db,
    s3: &S,
    x: &Book,
) -> Result<v1::book_index_response::Item> {
    let cover = match x.cover_id {
        Some(id) => {
            let it = AttachmentDao::by_id(db, id)?;
            let url = it.url(s3, Duration::hours(1))?;
            Some(url)
        }
        None => None,
    };
    Ok(v1::book_index_response::Item {
        id: x.id,
        name: x.name.clone(),
        description: x.description.clone(),
        deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
        updated_at: Some(to_timestamp!(x.updated_at)),
        cover,
    })
}

pub fn new_book_detail<S: S3>(db: &mut Db, s3: &S, x: &Book) -> Result<v1::BookDetail> {
    let cover = match x.cover_id {
        Some(id) => {
            let it = AttachmentDao::by_id(db, id)?;
            let url = it.url(s3, Duration::hours(1))?;
            Some(url)
        }
        None => None,
    };
    Ok(v1::BookDetail {
        id: x.id,
        name: x.name.clone(),
        deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
        cover,
    })
}
pub fn new_merchant_detail<S: S3>(db: &mut Db, s3: &S, x: &Merchant) -> Result<v1::MerchantDetail> {
    let cover = match x.cover_id {
        Some(id) => {
            let it = AttachmentDao::by_id(db, id)?;
            let url = it.url(s3, Duration::hours(1))?;
            Some(url)
        }
        None => None,
    };
    Ok(v1::MerchantDetail {
        id: x.id,
        name: x.name.clone(),
        deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
        cover,
    })
}
pub fn new_account_detail<S: S3>(db: &mut Db, s3: &S, x: &Account) -> Result<v1::AccountDetail> {
    let cover = match x.cover_id {
        Some(id) => {
            let it = AttachmentDao::by_id(db, id)?;
            let url = it.url(s3, Duration::hours(1))?;
            Some(url)
        }
        None => None,
    };
    Ok(v1::AccountDetail {
        id: x.id,
        name: x.name.clone(),
        deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
        cover,
        r#type: x.type_,
    })
}

fn new_user_detail(x: &User) -> v1::UserDetail {
    v1::UserDetail {
        id: x.id,
        name: x.name.clone().unwrap_or(EmailUser::GUEST_NAME.to_string()),
        avatar: x.avatar.clone(),
        deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
    }
}
