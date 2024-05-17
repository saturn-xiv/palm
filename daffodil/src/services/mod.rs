pub mod account;
pub mod book;
pub mod merchant;
pub mod transaction;

use camelia::{
    models::{attachment::Dao as AttachmentDao, user::Dao as UserDao},
    orm::postgresql::Connection as Db,
};
use chrono::Duration;
use palm::{daffodil::v1, jasmine::S3, to_timestamp, Result};

use super::models::{
    account::{Dao as AccountDao, Item as Account},
    book::{Dao as BookDao, Item as Book},
    merchant::{Dao as MerchantDao, Item as Merchant},
    transaction::{trash::Item as TransactionTrash, Item as Transaction},
};

pub fn new_transaction_trash_item<S: S3>(
    db: &mut Db,
    s3: &S,
    x: &TransactionTrash,
) -> Result<v1::transaction_trash_response::Item> {
    let source_account = AccountDao::by_id(db, x.source_account_id)?;
    let destination_account = AccountDao::by_id(db, x.destination_account_id)?;
    let user = UserDao::by_id(db, x.user_id)?;
    let operator = UserDao::by_id(db, x.operator_id)?;
    let book = BookDao::by_id(db, x.book_id)?;
    let merchant = MerchantDao::by_id(db, x.merchant_id)?;
    Ok(v1::transaction_trash_response::Item {
        id: x.id,
        reason: x.reason.clone(),
        operator: Some(operator.into()),
        original: Some(v1::transaction_index_response::Item {
            id: x.original_id,
            summary: x.summary.clone(),
            amount: Some(v1::Amount {
                currency: x.currency.clone(),
                value: x.amount,
            }),
            source: Some(new_account_detail(db, s3, &source_account)?),
            destination: Some(new_account_detail(db, s3, &destination_account)?),
            user: Some(user.into()),
            book: Some(new_book_detail(db, s3, &book)?),
            merchant: Some(new_merchant_detail(db, s3, &merchant)?),
            paid_at: Some(to_timestamp!(x.paid_at)),
            created_at: Some(to_timestamp!(x.original_created_at)),
        }),
        created_at: Some(to_timestamp!(x.created_at)),
    })
}
pub fn new_transaction_item<S: S3>(
    db: &mut Db,
    s3: &S,
    x: &Transaction,
) -> Result<v1::transaction_index_response::Item> {
    let source_account = AccountDao::by_id(db, x.source_account_id)?;
    let destination_account = AccountDao::by_id(db, x.destination_account_id)?;
    let user = UserDao::by_id(db, x.user_id)?;
    let book = BookDao::by_id(db, x.book_id)?;
    let merchant = MerchantDao::by_id(db, x.merchant_id)?;
    Ok(v1::transaction_index_response::Item {
        id: x.id,
        summary: x.summary.clone(),
        amount: Some(v1::Amount {
            currency: x.currency.clone(),
            value: x.amount,
        }),
        source: Some(new_account_detail(db, s3, &source_account)?),
        destination: Some(new_account_detail(db, s3, &destination_account)?),
        user: Some(user.into()),
        book: Some(new_book_detail(db, s3, &book)?),
        merchant: Some(new_merchant_detail(db, s3, &merchant)?),
        paid_at: Some(to_timestamp!(x.paid_at)),
        created_at: Some(to_timestamp!(x.created_at)),
    })
}
pub fn new_merchant_item<S: S3>(
    db: &mut Db,
    s3: &S,
    x: &Merchant,
) -> Result<v1::merchant_index_response::Item> {
    let cover = match x.cover_id {
        Some(id) => {
            let it = AttachmentDao::by_id(db, id)?;
            let url = it.url(s3, Duration::hours(1))?;
            Some(url)
        }
        None => None,
    };
    let user = UserDao::by_id(db, x.user_id)?;
    Ok(v1::merchant_index_response::Item {
        id: x.id,
        name: x.name.clone(),
        description: x.description.clone(),
        address: x.address.clone(),
        contact: x.contact.clone(),
        deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
        updated_at: Some(to_timestamp!(x.updated_at)),
        user: Some(user.into()),
        cover,
    })
}
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
        user: Some(user.into()),
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
