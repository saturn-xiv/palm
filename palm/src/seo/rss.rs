use chrono::{DateTime, NaiveDateTime, Utc};
use rss::ChannelBuilder as RssChannelBuilder;

use super::super::Result;

// https://validator.w3.org/feed/docs/rss2.html
pub struct Link {
    pub path: String,
    pub title: String,
    pub description: String,
    pub updated_at: NaiveDateTime,
}

pub fn build(home: &str, title: &str, description: &str, links: &[Link]) -> Result<Vec<u8>> {
    let pub_date: DateTime<Utc> = links.first().map_or_else(Utc::now, |x| {
        DateTime::<Utc>::from_naive_utc_and_offset(x.updated_at, Utc)
    });
    let mut ch = RssChannelBuilder::default()
        .title(title.to_string())
        .link(home.to_string())
        .pub_date(Some(pub_date.to_rfc2822()))
        .description(description.to_string())
        .build();
    for it in links {
        let url = format!("{}{}", home, it.path);
        ch.items.push(rss::Item {
            guid: Some(rss::Guid {
                value: url.clone(),
                permalink: true,
            }),
            title: Some(it.title.clone()),
            link: Some(url),
            description: Some(it.description.clone()),
            pub_date: Some(
                DateTime::<Utc>::from_naive_utc_and_offset(it.updated_at, Utc).to_rfc2822(),
            ),
            ..Default::default()
        });
    }
    let mut buf = Vec::new();
    ch.write_to(&mut buf)?;
    Ok(buf)
}
