pub mod about;
pub mod contact;
pub mod index;
pub mod services;

use std::path::Path;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    let root = Path::new("themes")
        .join("x-corporation-uiCookies")
        .join("HTML");
    let base = "/themes/x-corporation";
    config
        .service(
            actix_files::Files::new(&format!("{base}/img"), root.join("img"))
                .prefer_utf8(true)
                .use_etag(true)
                .use_last_modified(true),
        )
        .service(
            actix_files::Files::new(&format!("{base}/fonts"), root.join("fonts"))
                .prefer_utf8(true)
                .use_etag(true)
                .use_last_modified(true),
        )
        .service(
            actix_files::Files::new(&format!("{base}/css"), root.join("css"))
                .prefer_utf8(true)
                .use_etag(true)
                .use_last_modified(true),
        )
        .service(
            actix_files::Files::new(&format!("{base}/js"), root.join("js"))
                .prefer_utf8(true)
                .use_etag(true)
                .use_last_modified(true),
        );
}
