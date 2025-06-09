extern crate fig;

use chrono::Duration;
use fig::jwt::{Jwt as JwtProvider, openssl::Jwt};
use uuid::Uuid;

#[test]
fn jwt() {
    let audience = Uuid::new_v4().to_string();
    let subject = "sss";
    let issuer = "iii";

    let jwt = Jwt::new("O6tCeYbC+cpOzbEztjTLdZGtnLgWXRYpZdXmmftBKvY=".to_string());

    let token = jwt
        .sign(issuer, subject, &audience, Duration::weeks(1))
        .unwrap();
    println!("{}", token);
    {
        let it = jwt.verify(&token, issuer, &audience).unwrap();
        assert_eq!(subject, it);
    }

    assert!(jwt.verify(&token, "i1l", &audience).is_err());
    assert!(jwt.verify(&token, "i1l", "aaa").is_err());
}
