pub mod account;
pub mod category;
pub mod entry;
pub mod ledger;
pub mod merchant;
pub mod transaction;

use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

// Assets - Liabilities = Equity
// Assets - Liabilities = Equity + (Income - Expenses)
// Assets + Expenses = Liabilities + Income + Equity
#[derive(
    GraphQLEnum,
    EnumDisplay,
    EnumString,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
)]
#[serde(rename_all = "camelCase")]
#[graphql(name = "BookkeeperAccountType")]
pub enum AccountType {
    #[default]
    Cash,
    Bank,
    Stock,
    MutualFund,
    AccountsReceivable,
    OtherAssets,

    CreditCard,
    AccountsPayable,
    Liability,

    Equity,

    Income,

    Expenses,
}
