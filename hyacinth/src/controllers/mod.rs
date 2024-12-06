pub mod entries;
pub mod ledgers;

use actix_web::web;

pub fn html(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/accounting").service(
            web::scope("/ledgers")
                .service(ledgers::show)
                .service(ledgers::by_date_range)
                .service(ledgers::by_year_month)
                .service(ledgers::by_year)
                .service(ledgers::daily_by_date)
                .service(ledgers::weekly_by_date)
                .service(ledgers::monthly_by_date)
                .service(ledgers::yearly_by_date),
        ),
    );
}

pub fn api(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/accounting").service(web::scope("/entries").service(entries::bills_upload)),
    );
}
