pub mod pages;

use actix_web::web;

pub fn html(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/cms").service(
            web::scope("/pages")
                .service(pages::latest)
                .service(pages::by_day)
                .service(pages::by_month)
                .service(pages::by_year)
                .service(pages::upload),
        ),
    );
}

pub fn api(_config: &mut web::ServiceConfig) {}
