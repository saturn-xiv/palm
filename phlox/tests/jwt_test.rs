use chrono::Duration;
use phlox::{Jwt, base64, jwt::JwtHS512, random::bytes as random_bytes};

const ISSUER: &str = "i";
const SUBJECT: &str = "s";
const AUDIENCE: &str = "a";

#[test]
fn hs512() {
    let key = random_bytes(32);
    println!("key: {}", base64::encode(&key));
    let jwt = JwtHS512::new(&key);

    for i in 1..10 {
        let token = jwt
            .sign(
                ISSUER,
                SUBJECT,
                AUDIENCE,
                Duration::hours(1).to_std().unwrap(),
            )
            .unwrap();
        println!("jwt-hs512 token({}): {}", i, token);
        let subject = jwt.verify(&token, ISSUER, &[AUDIENCE]).unwrap();
        assert_eq!(SUBJECT, subject);
    }
}
