pub mod ledgers;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/{lang}/daffodil").service(web::scope("/ledgers").service(ledgers::show)),
    );
}
