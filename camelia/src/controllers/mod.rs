pub mod attachments;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(
        actix_files::Files::new("/3rd", "node_modules")
            .prefer_utf8(true)
            .use_etag(true)
            .use_last_modified(true),
    );
}
