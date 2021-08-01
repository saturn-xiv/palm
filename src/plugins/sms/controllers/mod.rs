pub mod log;

use actix_web::{web, Scope};

pub fn mount() -> Scope {
    web::scope("/twilio")
        .service(log::delivery_status)
        .service(log::incoming_messages)
}
