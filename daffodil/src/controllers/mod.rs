use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    if cfg!(debug_assertions) {
        config
            .service(
                actix_files::Files::new("/3rd", "node_modules")
                    .prefer_utf8(true)
                    .use_etag(true)
                    .use_last_modified(true),
            )
            .service(
                actix_files::Files::new("/public", "assets")
                    .prefer_utf8(true)
                    .use_etag(true)
                    .use_last_modified(true),
            );
    }
}
