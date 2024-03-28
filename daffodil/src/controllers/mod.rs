pub mod ledgers;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/{lang}/daffodil").service(
            web::scope("/ledgers")
                .service(ledgers::show::by_token::index)
                .service(ledgers::show::by_token::by_day)
                .service(ledgers::show::by_token::by_month)
                .service(ledgers::show::by_token::by_year),
        ),
    );
}
