pub mod baidu;
pub mod google;
pub mod index_now;
pub mod rss;
pub mod sitemap;

use robotxt::RobotsBuilder;

use super::Result;

// https://developers.google.com/search/docs/advanced/robots/create-robots-txt
pub fn robots_txt(home: &str) -> Result<String> {
    let buf = RobotsBuilder::default()
        .sitemap(format!("{home}/sitemap.xml").parse()?)
        .to_string();
    Ok(buf)
}
