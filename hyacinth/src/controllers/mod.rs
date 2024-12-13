pub mod entries;
pub mod ledgers;
pub mod statements;

use actix_web::web;

pub fn html(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/accounting").service(
            web::scope("/statements")
                .service(statements::by_date_range)
                .service(statements::by_year_month)
                .service(statements::by_year)
                .service(statements::daily_by_date)
                .service(statements::weekly_by_date)
                .service(statements::monthly_by_date)
                .service(statements::yearly_by_date),
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
