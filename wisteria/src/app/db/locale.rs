use std::collections::HashMap;
use std::path::Path;

use hyacinth::schema::locales;
use portal::{
    Result,
    models::locale::Dao as LocaleDao,
    orm::postgresql::{Connection as Db, Node as PostgreSql},
};
use serde::{Deserialize, Serialize};

pub fn sync<P: AsRef<Path>>(root: P) -> Result<(usize, usize)> {
    let root = root.as_ref();
    info!("load items from {}", root.display());

    let mut found = 0;
    let mut inserted = 0;

    for it in read_dir(root)? {
        let it = it?;
        let it = it.path();

        if it.is_dir() {
            if let Some(lang) = it.file_name() {
                if let Some(lang) = lang.to_str() {
                    info!("find language {}", lang);
                    for it in read_dir(&it)? {
                        let it = it?;
                        let it = it.path();
                        if it.is_file() {
                            let (f, i) = load_from_yaml(self, lang, it)?;
                            found += f;
                            inserted += i;
                        }
                    }
                }
            }
        }
    }
    for lang in self.languages()?.iter() {
        for it in read_dir(root)? {
            let it = it?;
            let it = it.path();

            if it.is_file() {
                let (f, i) = load_from_yaml(self, lang, it)?;
                found += f;
                inserted += i;
            }
        }
    }

    info!("sync {}/{} items", inserted, found);
    Ok((inserted, found))
}

fn load_from_yaml<P: AsRef<Path>>(db: &mut Db, lang: &str, file: P) -> Result<(usize, usize)> {
    let mut found = 0;
    let mut inserted = 0;
    let file = file.as_ref();
    info!("find file {}", file.display());

    for (sec, props) in Ini::load_from_file(file)?.iter() {
        if let Some(sec) = sec {
            debug!("find section {}", sec);
            for (key, val) in props.iter() {
                let code = format!("{}.{}", sec, key);
                debug!("find {lang}.{code} = {val}");
                found += 1;
                let cnt: i64 = locales::dsl::locales
                    .count()
                    .filter(locales::dsl::lang.eq(lang))
                    .filter(locales::dsl::code.eq(&code))
                    .get_result(db)?;
                if cnt == 0 {
                    LocaleDao::create(db, lang, &code, val)?;
                    inserted += 1;
                } else {
                    warn!("{lang}.{code} already exists!");
                }
            }
        }
    }

    Ok((found, inserted))
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
enum Item {
    S(String),
    M(HashMap<String, String>),
    O(HashMap<String, Self>),
}
