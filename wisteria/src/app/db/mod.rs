pub mod locale;

use std::{
    ops::DerefMut,
    path::{Path, PathBuf},
};

use diesel::Connection as DieselConnection;
use portal::{
    Error, Result, iso4217::ISO4217, models::currency::Dao as CurrencyDao,
    orm::postgresql::Node as PostgreSql, parse_toml,
};
use serde::{Deserialize, Serialize};

pub async fn seeds<P: AsRef<Path>>(config: P, locales: Option<Vec<PathBuf>>) -> Result<()> {
    let config: Config = parse_toml(config)?;
    let db = config.postgresql.open()?;
    let mut db = db.get()?;
    let db = db.deref_mut();

    if let Some(ref locales) = locales {
        db.transaction::<_, Error, _>(|tx| {
            let mut found = 0;
            let mut inserted = 0;
            for it in locales {
                let (f, i) = locale::sync(tx, it)?;
                found += f;
                inserted += i;
            }
            log::info!("sync {}/{} locale items", found, inserted);
            Ok(())
        })?;
    }

    if CurrencyDao::count(db)? == 0 {
        let iso4217 = ISO4217::new()?;
        db.transaction::<_, Error, _>(|tx| {
            for it in iso4217.table.items.iter() {
                if let Some(ref code) = it.code
                    && let Some(ref number) = it.number
                {
                    log::debug!(
                        "found {}({}) {}",
                        it.name.value,
                        code.value,
                        it.country.value
                    );
                    CurrencyDao::create(
                        tx,
                        &it.name.value,
                        &code.value,
                        &it.country.value,
                        number.value as i32,
                        it.units()?.map(|x| x as i32),
                        it.name.fund,
                    )?;
                }
            }
            Ok(())
        })?;
    }

    log::info!("done.");
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    postgresql: PostgreSql,
}
