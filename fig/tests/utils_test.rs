use std::any::type_name;

use fig::{jwt::openssl::Jwt, palm::email::v1::EmailTask};

#[test]
fn name() {
    println!("{}", type_name::<EmailTask>());
    println!("{}", type_name::<Jwt>());
}
