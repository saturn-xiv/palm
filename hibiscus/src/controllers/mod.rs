pub mod forums;
pub mod posts;
pub mod topics;

use actix_web::web;

pub fn html(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/bbs")
            .service(
                web::scope("/forums")
                    .service(forums::index)
                    .service(forums::by_code),
            )
            .service(
                web::scope("/topics")
                    .service(topics::latest)
                    .service(topics::by_day)
                    .service(topics::by_month)
                    .service(topics::by_year)
                    .service(topics::by_code),
            )
            .service(
                web::scope("/posts")
                    .service(posts::latest)
                    .service(posts::by_day)
                    .service(posts::by_month)
                    .service(posts::by_year),
            ),
    );
}

pub fn api(_config: &mut web::ServiceConfig) {}
