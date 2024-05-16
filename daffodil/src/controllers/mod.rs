pub mod books;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(web::scope("/{lang}/daffodil").service(
        web::scope("/books").service(books::show::by_token::index), // .service(books::show::by_token::by_day)
                                                                    // .service(books::show::by_token::by_month)
                                                                    // .service(books::show::by_token::by_year),
    ));
}
