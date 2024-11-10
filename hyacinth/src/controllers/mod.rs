pub mod ledgers;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/bookkeeper").service(
            web::scope("/ledgers")
                .service(ledgers::by_year)
                .service(ledgers::by_year_month)
                .service(ledgers::by_date_range)
                .service(ledgers::daily_by_date)
                .service(ledgers::weekly_by_date)
                .service(ledgers::monthly_by_date)
                .service(ledgers::yearly_by_date),
        ),
    );
}
