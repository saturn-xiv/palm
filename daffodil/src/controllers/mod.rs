pub mod admin;
pub mod attachments;
pub mod home;
pub mod wang_editor;

use actix_web::web;

pub fn html(config: &mut web::ServiceConfig) {
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

    config.service(home::get);
}

pub fn api(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/attachments").service(attachments::show))
        .service(web::scope("/admin").service(web::scope("/site").service(admin::site::favicon)))
        .service(
            web::scope("/wang-editor").service(
                web::scope("/upload")
                    .service(wang_editor::upload::image)
                    .service(wang_editor::upload::video),
            ),
        );
}
