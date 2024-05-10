use chrono::NaiveDateTime;
use palm::{to_utc_datetime, Result};
use sitewriter::{ChangeFreq, UrlEntryBuilder};

// https://www.sitemaps.org/protocol.html#
pub struct Link {
    pub path: String,
    pub change_freq: ChangeFreq,
    pub priority: f32,
    pub updated_at: NaiveDateTime,
}

pub fn urlset(home: &str, links: &[Link]) -> Result<String> {
    let mut items = Vec::new();
    for it in links {
        let url = format!("{}{}", home, it.path);
        items.push(
            UrlEntryBuilder::default()
                .loc(url.parse()?)
                .changefreq(it.change_freq)
                .priority(it.priority)
                .lastmod(to_utc_datetime!(it.updated_at))
                .build()?,
        )
    }
    let buf = sitewriter::generate_str(&items);
    Ok(buf)
}
