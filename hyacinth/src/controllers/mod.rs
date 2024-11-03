pub mod ledgers;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/bookkeeper").service(
            web::scope("/ledgers")
                .service(ledgers::by_year )
                .service(ledgers::by_year_month)
                .service(ledgers::by_year_month_day),
        ),
    );
}
