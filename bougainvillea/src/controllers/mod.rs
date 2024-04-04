pub mod logs;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(web::scope("/logs").service(logs::index));
}
