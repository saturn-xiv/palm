use std::fs::{File, read_dir};
use std::path::Path;

use diesel::prelude::*;
use hyacinth::schema::locales;
use hyper::StatusCode;
use icu::locale::Locale;
use portal::{
    HttpError, Result, models::locale::Dao as LocaleDao, orm::postgresql::Connection as Db,
};
use yaml_serde::{Mapping as YamlMapping, Value as YamlValue, from_reader as yaml_from_reader};

pub fn sync<P: AsRef<Path>>(db: &mut Db, root: P) -> Result<(usize, usize)> {
    let root = root.as_ref();
    log::info!("load items from {}", root.display());

    let mut found = 0;
    let mut inserted = 0;

    for it in read_dir(root)? {
        let it = it?;
        let it = it.path();

        if it.is_dir()
            && let Some(lang) = it.file_name()
            && let Some(lang) = lang.to_str()
        {
            let lang = lang.parse()?;
            log::info!("find language {}", lang);
            for it in read_dir(&it)? {
                let it = it?;
                let it = it.path();
                if it.is_file() {
                    let (f, i) = load_from_yaml(db, &lang, it)?;
                    found += f;
                    inserted += i;
                }
            }
        }
    }
    for lang in LocaleDao::languages(db)?.iter() {
        let lang = lang.parse()?;
        for it in read_dir(root)? {
            let it = it?;
            let it = it.path();

            if it.is_file() {
                let (f, i) = load_from_yaml(db, &lang, it)?;
                found += f;
                inserted += i;
            }
        }
    }

    Ok((inserted, found))
}

fn load_from_yaml<P: AsRef<Path>>(db: &mut Db, lang: &Locale, file: P) -> Result<(usize, usize)> {
    let file = file.as_ref();
    {
        let it = file.extension().and_then(|x| x.to_str()).ok_or_else(|| {
            Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("could't detect extension name".to_string()),
            ))
        })?;
        if !["yaml", "yml"].contains(&it) {
            log::warn!("ignore file {} for lang {}", file.display(), lang);
            return Ok((0, 0));
        }
    }
    log::info!("find file {}", file.display());
    let section = file.file_stem().and_then(|x| x.to_str()).ok_or_else(|| {
        Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("could't detect file stem name".to_string()),
        ))
    })?;

    let file = File::open(file)?;

    let root: YamlValue = yaml_from_reader(file)?;
    let root = root.as_mapping().ok_or_else(|| {
        Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("invalid locale yaml file".to_string()),
        ))
    })?;

    load_from_yaml_mapping(db, lang, section, root)
}

fn load_from_yaml_mapping(
    db: &mut Db,
    lang: &Locale,
    section: &str,
    mapping: &YamlMapping,
) -> Result<(usize, usize)> {
    let mut found = 0;
    let mut inserted = 0;

    for (key, val) in mapping {
        let key = key.as_str().ok_or_else(|| {
            Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("invalid locale yaml key".to_string()),
            ))
        })?;
        let key = format!("{section}.{key}");
        match val {
            YamlValue::Mapping(mapping) => {
                let (f, i) = load_from_yaml_mapping(db, lang, &key, mapping)?;
                found += f;
                inserted += i;
                Ok(())
            }
            YamlValue::String(message) => {
                let (f, i) = save_locale_item(db, lang, &key, message)?;
                found += f;
                inserted += i;
                Ok(())
            }
            _ => Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("invalid locale yaml value".to_string()),
            ))),
        }?;
    }
    Ok((found, inserted))
}

fn save_locale_item(
    db: &mut Db,
    lang: &Locale,
    code: &str,
    message: &str,
) -> Result<(usize, usize)> {
    log::debug!("find {lang}.{code}");

    {
        let lang = lang.to_string();
        let cnt: i64 = locales::dsl::locales
            .count()
            .filter(locales::dsl::lang.eq(&lang))
            .filter(locales::dsl::code.eq(code))
            .get_result(db)?;
        if cnt > 0 {
            log::debug!("{lang}.{code} already exists!");
            return Ok((1, 0));
        }
    }
    LocaleDao::create(db, lang, code, message)?;
    Ok((1, 1))
}
