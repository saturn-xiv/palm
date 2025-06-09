use chrono::Duration;
use data_encoding::BASE64_NOPAD;
use fig::{
    crypto::{Password, Secret},
    jwt::Jwt,
    thrift::Thrift,
};
use uuid::Uuid;

#[test]
fn tink() {
    let cli = Thrift {
        host: "127.0.0.1".to_string(),
        port: 2345,
        tls: None,
    };

    {
        let audience = Uuid::new_v4().to_string();
        let subject = "sss";
        let issuer = "iii";

        let token = Jwt::sign(&cli, issuer, subject, &audience, Duration::weeks(1)).unwrap();
        println!("{}", token);
        {
            let it = Jwt::verify(&cli, &token, issuer, &audience).unwrap();
            assert_eq!(subject, it);
        }

        assert!(Jwt::verify(&cli, &token, "i1l", &audience).is_err());
        assert!(Jwt::verify(&cli, &token, "i1l", "aaa").is_err());
    }

    let plain = "hi, palm!";

    {
        let (code, nonce) = Secret::encrypt(&cli, plain.as_bytes()).unwrap();
        println!(
            "AES: {} {}",
            BASE64_NOPAD.encode(&code),
            BASE64_NOPAD.encode(&nonce),
        );

        let it = Secret::decrypt(&cli, &code, &nonce).unwrap();
        assert_eq!(plain, std::str::from_utf8(&it).unwrap());
    }

    {
        let code = Password::sign(&cli, plain.as_bytes()).unwrap();
        println!("HMAC: {}", BASE64_NOPAD.encode(&code));
        assert!(Password::verify(&cli, &code, plain.as_bytes()));
        assert!(!Password::verify(&cli, "aaa".as_bytes(), plain.as_bytes()));
        assert!(!Password::verify(&cli, &code, "hello".as_bytes()));
    }
}
