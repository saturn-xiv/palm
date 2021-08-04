use eui48::MacAddress;
use serde::{Deserialize, Serialize};

use super::super::super::super::{
    crypto::Aes, jwt::Jwt, orm::sqlite::Connection as Db, request::Token, Result,
};
use super::super::models::settings::Dao as SettingDao;
use super::user::CurrentUser;
