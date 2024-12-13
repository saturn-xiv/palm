pub mod cover;

use chrono::{Datelike, Months, NaiveDate, NaiveDateTime, NaiveTime};
use petunia::orm::postgresql::Connection as Db;

use super::super::{
    layout::bootstrap5::{Dropdown, Link, NavBar},
    models::{ledger::Item as Ledger, transaction::Dao as TransactionDao},
};

impl Link {
    fn by_date(home: &str, it: NaiveDateTime) -> Self {
        Self {
            label: it.format("%Y-%m").to_string(),
            to: format!("{}by-month/{}-{}", home, it.year(), it.month()),
        }
    }
}

impl Dropdown {
    fn by_date_range(home: &str, begin: NaiveDateTime, end: NaiveDateTime) -> Vec<Self> {
        let mut items = Vec::new();
        items.push(Self::by_year_month(home, end, begin.year(), begin.month()));
        {
            let mut i = 1;
            loop {
                i += 1;
                let it = Self::by_year_month(home, end, begin.year() + i, 1);
                if it.items.is_empty() {
                    break;
                }
                items.push(it);
            }
        }
        items
    }

    fn by_year_month(home: &str, end: NaiveDateTime, year: i32, month: u32) -> Self {
        let mut items = Vec::new();

        if let Some(begin) = NaiveDate::from_ymd_opt(year, month, 1) {
            let begin = begin.and_time(NaiveTime::MIN);
            if begin < end {
                items.push(Link::by_date(home, begin));
            }
            if begin.month() < 12 {
                let mut i = 1;
                loop {
                    i += 1;
                    if let Some(it) = begin.checked_add_months(Months::new(i)) {
                        if it > end {
                            break;
                        }
                        items.push(Link::by_date(home, it));
                        if it.month() == 12 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Self {
            items,
            label: format!("{year}"),
        }
    }
}

impl NavBar {
    pub fn by_ledger(db: &mut Db, ledger: &Ledger, home: &str) -> Self {
        let begin = match TransactionDao::first_by_ledger(db, ledger.id) {
            Ok(it) => it.traded_at,
            Err(e) => {
                log::error!("{:?}", e);
                ledger.created_at
            }
        };
        let end = match TransactionDao::last_by_ledger(db, ledger.id) {
            Ok(it) => it.traded_at,
            Err(e) => {
                log::error!("{:?}", e);
                ledger.created_at
            }
        };

        Self {
            items: Dropdown::by_date_range(home, begin, end),
        }
    }
}
