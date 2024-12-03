use std::str::FromStr;

use chrono::{offset::LocalResult, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

use super::{Error, HttpError, Result};

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct DateTimePicker {
    // https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    // '2024-12-09 01:02:03'
    pub datetime: String,
    pub timezone: String,
}

impl DateTimePicker {
    // "%Y-%m-%d %H:%M:%S"
    const FORMAT: &str = "%F %T";
}

impl TryFrom<(NaiveDateTime, Tz)> for DateTimePicker {
    type Error = Error;
    fn try_from((value, tz): (NaiveDateTime, Tz)) -> Result<Self> {
        let it = Self {
            datetime: tz
                .from_utc_datetime(&value)
                .format(Self::FORMAT)
                .to_string(),
            timezone: tz.to_string(),
        };
        Ok(it)
    }
}

impl TryFrom<DateTimePicker> for (NaiveDateTime, Tz) {
    type Error = Error;

    fn try_from(value: DateTimePicker) -> Result<Self> {
        let tz = Tz::from_str(&value.timezone)?;

        if let LocalResult::Single(it) =
            NaiveDateTime::parse_from_str(&value.datetime, DateTimePicker::FORMAT)?
                .and_local_timezone(tz)
        {
            return Ok((it.naive_utc(), tz));
        }
        Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)))
    }
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct Succeed {
    pub created_at: NaiveDateTime,
}
impl Default for Succeed {
    fn default() -> Self {
        Self {
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub total: i32,
    pub size: i32,
    pub has_next: bool,
    pub has_previous: bool,
}

impl Pagination {
    pub fn new(pager: &Pager, total: i64) -> Self {
        let page = pager.page(total);
        let size = pager.size();

        Self {
            page: page as i32,
            size: size as i32,
            total: total as i32,
            has_next: (page * size < total),
            has_previous: (page > 1),
        }
    }
}

#[derive(GraphQLInputObject, Debug, Serialize, Deserialize)]
pub struct Pager {
    pub page: i32,
    pub size: i32,
}

impl Pager {
    pub fn offset(&self, total: i64) -> i64 {
        (self.page(total) - 1) * self.size()
    }

    pub fn page(&self, total: i64) -> i64 {
        let size = self.size();
        if total < size || self.page < 1 {
            return 1;
        }
        let page = self.page as i64;
        if page * size > total {
            let it = total / size;
            return if total % size == 0 { it } else { it + 1 };
        }
        page
    }

    pub fn size(&self) -> i64 {
        let size = self.size as i64;
        if size < Self::MIN_SIZE {
            return Self::MIN_SIZE;
        }
        if size > Self::MAX_SIZE {
            return Self::MAX_SIZE;
        }
        size
    }
    const MAX_SIZE: i64 = 1 << 12;
    const MIN_SIZE: i64 = 1 << 2;
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct Resource {
    pub r#type: String,
    pub id: Option<i32>,
}
