pub mod list_one;

// https://www.iso.org/iso-4217-currency-codes.html

use std::collections::HashSet;

use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use super::{HttpError, Result};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Currency {
    pub id: i32,
    pub code: String,
    pub unit: i32,
    pub name: String,
}

impl Currency {
    pub fn all() -> Result<Vec<Self>> {
        let mut items = HashSet::new();
        for it in list_one::Item::all()?.iter() {
            if let Some(id) = it.id {
                if let Some(ref code) = it.code {
                    if let Some(unit) = it.unit {
                        let it = Self {
                            code: code.clone(),
                            name: it.name.clone(),
                            unit,
                            id,
                        };
                        items.insert(it);
                    }
                }
            }
        }

        Ok(items.into_iter().collect())
    }
    pub fn new(code: &str) -> Result<Self> {
        Self::all()?
            .into_iter()
            .filter(|x| x.code == code)
            .nth(0)
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("unknown currency code {code}")),
            )))
    }
}
