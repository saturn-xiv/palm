use askama::Template;
use chrono::{Duration, NaiveDateTime};
use daffodil::{
    models::{attachment::Dao as AttachmentDao, locale::Dao as LocaleDao},
    schema::currencies,
};
use diesel::prelude::*;
use petunia::{orm::postgresql::Connection as Db, s3::Client as S3, Result, GIT_VERSION};
use serde::{Deserialize, Serialize};

use super::super::super::{
    layout::bootstrap5::{Layout, NavBar},
    models::{entry::Item as Entry, ledger::Item as Ledger, statement::Item as Statement},
    schema::bookkeeper_statements,
};

#[derive(Template)]
#[template(path = "statements/index.html")]
pub struct Index {
    pub ledger: Ledger,
    pub layout: Layout,
    pub statements: Vec<Item>,
}

impl Index {
    pub async fn new(
        db: &mut Db,
        s3: &S3,
        ledger: &Ledger,
        lang: &str,
        home: &str,
        (begin, end, ttl): (NaiveDateTime, NaiveDateTime, Option<Duration>),
    ) -> Result<Self> {
        Ok(Self {
            layout: Layout {
                title: ledger.label.clone(),
                nav_bar: NavBar::by_ledger(db, ledger, home),
                home: home.to_string(),
                locales: LocaleDao::map_by_lang(db, lang)?,
                version: GIT_VERSION.to_string(),
            },
            ledger: ledger.clone(),
            statements: Item::by_ledger(db, s3, ledger.id, (begin, end, ttl)).await?,
        })
    }
}

/**

借贷记账法是会计核算的基本原则之一，也称为“双向记账法”。
它的核心思想是每一笔交易都涉及到至少两个账户，一个账户增加，另一个账户减少，这样会计记录的总借方等于总贷方，保持会计方程式平衡。

在借贷记账法中，记账人需要将每一笔交易分解成借方和贷方两个部分，具体的规则如下：
1. 借方：表示资产的增加，费用的减少，负债和所有者权益的减少。
2. 贷方：表示资产的减少，费用的增加，负债和所有者权益的增加。

例如，公司购进一批原材料，假设花费了1万元，那么会计记录应该如下：
借：原材料 1万元
贷：银行存款 1万元
这个例子中，原材料账户增加1万元，银行存款账户减少1万元，符合借贷记账法的规则。

在实际操作中，会计人员需要熟练掌握借贷记账法的应用，确保每一笔交易的记录准确无误，从而保证公司的财务状况真实可靠。

 */

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub item: Statement,
    pub invoices: Vec<Invoice>,
    pub amount: Amount,
    pub debtor_opening_balance: Amount,
    pub debtor_closing_balance: Amount,
    pub creditor_opening_balance: Amount,
    pub creditor_closing_balance: Amount,
}

impl Item {
    pub async fn by_ledger(
        db: &mut Db,
        s3: &S3,
        ledger: i32,
        (begin, end, ttl): (NaiveDateTime, NaiveDateTime, Option<Duration>),
    ) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        for item in bookkeeper_statements::dsl::bookkeeper_statements
            .filter(bookkeeper_statements::dsl::ledger_id.eq(ledger))
            .filter(bookkeeper_statements::dsl::created_at.ge(begin))
            .filter(bookkeeper_statements::dsl::created_at.lt(end))
            .order(bookkeeper_statements::dsl::created_at.desc())
            .load::<Statement>(db)?
        {
            let it = Self {
                invoices: {
                    let mut items = Vec::new();
                    for it in AttachmentDao::by_resource::<Entry>(db, Some(item.entry_id))?.iter() {
                        items.push(Invoice {
                            title: it.title.clone(),
                            content_type: it.content_type.clone(),
                            url: it.url(s3, ttl).await?,
                        });
                    }
                    items
                },
                amount: Amount::new(db, item.category_id, item.amount)?,
                debtor_opening_balance: Amount::new(
                    db,
                    item.category_id,
                    item.debtor_opening_balance,
                )?,
                debtor_closing_balance: Amount::new(
                    db,
                    item.category_id,
                    item.debtor_closing_balance,
                )?,
                creditor_opening_balance: Amount::new(
                    db,
                    item.category_id,
                    item.creditor_opening_balance,
                )?,
                creditor_closing_balance: Amount::new(
                    db,
                    item.category_id,
                    item.creditor_closing_balance,
                )?,
                item,
            };
            items.push(it);
        }

        Ok(items)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub url: String,
    pub title: String,
    pub content_type: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Transaction {
//     pub id: i32,
//     pub uid: String,
//     pub memo: String,
//     pub updated_at: NaiveDateTime,
// }
// impl Transaction {
//     pub async fn by_ledger(db: &mut Db, s3: &S3, ledger: i32, ttl: Duration) -> Result<Vec<Self>> {
//         let mut items = Vec::new();

//         for (tid, uid, memo, traded_at, timezone, updated_at) in
//             bookkeeper_transactions::dsl::bookkeeper_transactions
//                 .select((
//                     bookkeeper_transactions::dsl::id,
//                     bookkeeper_transactions::dsl::uid,
//                     bookkeeper_transactions::dsl::memo,
//                     bookkeeper_transactions::dsl::traded_at,
//                     bookkeeper_transactions::dsl::timezone,
//                     bookkeeper_transactions::dsl::updated_at,
//                 ))
//                 .filter(bookkeeper_transactions::dsl::ledger_id.eq(ledger))
//                 .order(bookkeeper_transactions::dsl::traded_at.desc())
//                 .load::<(i32, String, String, NaiveDateTime, String, NaiveDateTime)>(db)?
//         {
//             let it = Self {
//                 uid,
//                 memo,
//                 traded_at,
//                 updated_at,
//                 timezone: Tz::from_str(&timezone)?,
//                 entries: Entry::by_transaction(db, s3, tid, ttl).await?,
//             };
//             items.push(it);
//         }
//         Ok(items)
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Entry {
//     pub id: i32,
//     pub sn: String,
//     pub memo: String,
//     pub traded_at: NaiveDateTime,
//     pub timezone: Tz,
//     pub from_account: Account,
//     pub to_account: Account,
//     pub category: Category,
//     pub merchant: Merchant,
//     pub amount: Amount,
//     pub updated_at: NaiveDateTime,
//     pub invoices: Vec<Invoice>,
// }

// impl Entry {
//     async fn new(db: &mut Db, s3: &S3, id: i32, ttl: Duration) -> Result<Vec<Self>> {
//         let mut items = Vec::new();
//         let (
//             id,
//             sn,
//             memo,
//             traded_at,
//             timezone,
//             from_account,
//             to_account,
//             category,
//             merchant,
//             amount,
//             updated_at,
//         ) = bookkeeper_entries::dsl::bookkeeper_entries
//             .select((
//                 bookkeeper_entries::dsl::id,
//                 bookkeeper_entries::dsl::sn,
//                 bookkeeper_entries::dsl::memo,
//                 bookkeeper_entries::dsl::traded_at,
//                 bookkeeper_entries::dsl::timezone,
//                 bookkeeper_entries::dsl::from_account_id,
//                 bookkeeper_entries::dsl::to_account_id,
//                 bookkeeper_entries::dsl::category_id,
//                 bookkeeper_entries::dsl::merchant_id,
//                 bookkeeper_entries::dsl::amount,
//                 bookkeeper_entries::dsl::updated_at,
//             ))
//             .filter(bookkeeper_entries::dsl::id.eq(id))
//             .limit(1)
//             .first::<(
//                 i32,
//                 String,
//                 String,
//                 NaiveDateTime,
//                 String,
//                 i32,
//                 i32,
//                 i32,
//                 i32,
//                 i32,
//                 NaiveDateTime,
//             )>(db)?;
//         {
//             let currency = bookkeeper_accounts::dsl::bookkeeper_accounts
//                 .select(bookkeeper_accounts::dsl::currency_id)
//                 .filter(bookkeeper_accounts::dsl::id.eq(from_account))
//                 .first::<i32>(db)?;
//             let it = Self {
//                 id,
//                 sn,
//                 memo,
//                 traded_at,
//                 updated_at,
//                 timezone: Tz::from_str(&timezone)?,
//                 from_account: Account::new(db, from_account)?,
//                 to_account: Account::new(db, to_account)?,
//                 category: Category::new(db, category)?,
//                 merchant: Merchant::new(db, merchant)?,
//                 amount: Amount::new(db, currency, amount)?,
//                 invoices: Invoice::by_entry(db, s3, id, ttl).await?,
//             };
//             items.push(it);
//         }
//         Ok(items)
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Account {
//     pub id: i32,
//     pub label: String,
//     pub memo: String,
// }

// impl Account {
//     pub fn new(db: &mut Db, id: i32) -> Result<Self> {
//         let (label, memo) = bookkeeper_accounts::dsl::bookkeeper_accounts
//             .select((
//                 bookkeeper_accounts::dsl::label,
//                 bookkeeper_accounts::dsl::memo,
//             ))
//             .filter(bookkeeper_accounts::dsl::id.eq(id))
//             .first::<(String, String)>(db)?;
//         Ok(Self { id, label, memo })
//     }
// }
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub country: String,
    pub units: i32,
}

impl Currency {
    pub fn new(db: &mut Db, currency: i32) -> Result<Self> {
        let (id, code, name, country, units) = currencies::dsl::currencies
            .select((
                currencies::dsl::id,
                currencies::dsl::code,
                currencies::dsl::name,
                currencies::dsl::country,
                currencies::dsl::units,
            ))
            .filter(currencies::dsl::id.eq(currency))
            .first::<(i32, String, String, String, i32)>(db)?;
        Ok(Self {
            id,
            code,
            name,
            country,
            units,
        })
    }
}
// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Category {
//     pub id: i32,
//     pub label: String,
// }

// impl Category {
//     pub fn new(db: &mut Db, id: i32) -> Result<Self> {
//         let label = bookkeeper_categories::dsl::bookkeeper_categories
//             .select(bookkeeper_categories::dsl::label)
//             .filter(bookkeeper_categories::dsl::id.eq(id))
//             .first::<String>(db)?;
//         Ok(Self { id, label })
//     }
// }
// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Merchant {
//     pub id: i32,
//     pub label: String,
//     pub memo: String,
// }

// impl Merchant {
//     pub fn new(db: &mut Db, id: i32) -> Result<Self> {
//         let (label, memo) = bookkeeper_merchants::dsl::bookkeeper_merchants
//             .select((
//                 bookkeeper_merchants::dsl::label,
//                 bookkeeper_merchants::dsl::memo,
//             ))
//             .filter(bookkeeper_merchants::dsl::id.eq(id))
//             .first::<(String, String)>(db)?;
//         Ok(Self { id, label, memo })
//     }
// }
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub value: String,
    pub currency: Currency,
}

impl Amount {
    pub fn new(db: &mut Db, currency: i32, value: i32) -> Result<Self> {
        let currency = Currency::new(db, currency)?;
        let it = Self {
            value: {
                let x = value as f64;
                let y = 10_i32.pow(currency.units as u32) as f64;
                let v = x / y;
                match currency.units {
                    3 => format!("{:.3}", v),
                    _ => format!("{:.2}", v),
                }
            },
            currency,
        };
        Ok(it)
    }
}
// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Invoice {
//     pub id: i32,
//     pub title: String,
//     pub content_type: String,
//     pub bucket: String,
//     pub object: String,
//     pub size: i32,
//     pub url: String,
// }

// impl Invoice {
//     pub async fn by_entry(db: &mut Db, s3: &S3, entry: i32, ttl: Duration) -> Result<Vec<Self>> {
//         let mut items = Vec::new();
//         let ids: Vec<i32> = attachment_resources::dsl::attachment_resources
//             .select(attachment_resources::dsl::attachment_id)
//             .filter(attachment_resources::dsl::resource_type.eq(type_name::<EntryItem>()))
//             .filter(attachment_resources::dsl::resource_id.eq(entry))
//             .order(attachment_resources::dsl::created_at.desc())
//             .load(db)?;
//         for (id, title, content_type, bucket, object, size) in attachments::dsl::attachments
//             .select((
//                 attachments::dsl::id,
//                 attachments::dsl::title,
//                 attachments::dsl::content_type,
//                 attachments::dsl::bucket,
//                 attachments::dsl::object,
//                 attachments::dsl::size,
//             ))
//             .filter(attachments::dsl::id.eq_any(ids))
//             .load::<(i32, String, String, String, String, i32)>(db)?
//         {
//             let it = Self {
//                 id,
//                 url: s3
//                     .get_object_url(&title, &content_type, &bucket, &object, Some(ttl))
//                     .await?,
//                 title,
//                 content_type,
//                 bucket,
//                 object,
//                 size,
//             };
//             items.push(it);
//         }
//         Ok(items)
//     }
// }
