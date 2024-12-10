pub mod entries;
pub mod ledgers;

use actix_web::web;

pub fn html(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/accounting").service(
            web::scope("/ledgers")
                .service(ledgers::show::get)
                .service(ledgers::bills::by_date_range)
                .service(ledgers::bills::by_year_month)
                .service(ledgers::bills::by_year)
                .service(ledgers::bills::daily_by_date)
                .service(ledgers::bills::weekly_by_date)
                .service(ledgers::bills::monthly_by_date)
                .service(ledgers::bills::yearly_by_date),
        ),
    );
}

pub fn api(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/accounting")
            .service(web::scope("/ledgers").service(ledgers::cover::save))
            .service(web::scope("/entries").service(entries::bills_upload)),
    );
}
