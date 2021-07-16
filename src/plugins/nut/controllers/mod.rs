// pub mod attachments;

// use std::sync::Arc;

use askama::Template;

// use hyper::header::AUTHORIZATION;

// use super::super::super::{crypto::Aes, jwt::Jwt, orm::postgresql::Pool as DbPool};

// // https://developers.google.com/search/docs/advanced/robots/create-robots-txt
// pub fn robots_txt(state: State) -> (State, Response<Body>) {
//     let tpl = RobotsTxt { domain: "todo!" };
//     let res = __plain_text!(&state, tpl.render());
//     // TODO
//     (state, res)
// }

// // https://developers.google.com/search/docs/advanced/sitemaps/build-sitemap#xml
// pub fn sitemap_xml_gz(state: State) -> (State, Response<Body>) {
//     let tpl = Home {};
//     let res = __plain_text!(&state, tpl.render());
//     // TODO
//     (state, res)
// }

// // https://cyber.harvard.edu/rss/rss.html
// pub fn rss_xml(state: State) -> (State, Response<Body>) {
//     let tpl = Home {};
//     let res = __plain_text!(&state, tpl.render());
//     // TODO
//     (state, res)
// }

#[derive(Template)]
#[template(path = "robots.txt", escape = "none")]
pub struct RobotsTxt<'a> {
    pub domain: &'a str,
}
