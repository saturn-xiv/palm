use std::collections::BTreeSet;
use std::env;
use std::thread;
use std::time::Duration as StdDuration;

use chrono::Duration;
use data_encoding::BASE64;
use palm::{
    daisy::protocols::{Address, Body, EmailSendTask},
    from_bytes,
    morus::markdown::Markdown,
    to_bytes, Jwt, Password, Secret, Thrift,
};

#[test]
fn loquat() {
    let client = Thrift {
        host: env::var("LOQUAT_HOST").unwrap(),
        port: env::var("LOQUAT_PORT").unwrap().parse().unwrap(),
        secure: false,
    };
    println!("connect to loquat {}", client);

    let hi = "hello, palm!";
    for _ in 1..5 {
        let issuer = "i";
        let subject = "s";
        let audience = "a";
        let token = Jwt::sign_by_duration(&client, issuer, subject, audience, Duration::minutes(5))
            .unwrap();
        println!("receive jwt token: {}", token);
        {
            let (jwt_id, sub) = Jwt::verify(&client, &token, issuer, audience).unwrap();
            println!("jwt id: {}, sub: {}", jwt_id, sub);
            assert_eq!(subject, sub);
        }
        thread::sleep(StdDuration::from_secs(1));
    }
    for _ in 1..5 {
        let (password, salt) = Password::compute(&client, hi.as_bytes(), 8).unwrap();
        println!("password: {}", BASE64.encode(&password));
        println!("salt: {}", BASE64.encode(&salt));
        assert!(Password::verify(&client, &password, hi.as_bytes(), &salt));
    }
    for _ in 1..5 {
        let (code, iv) = Secret::encrypt(&client, hi.as_bytes()).unwrap();
        println!("code: {}", BASE64.encode(&code));
        println!("iv: {}", BASE64.encode(&iv));
        {
            let plain = Secret::decrypt(&client, &code, &iv).unwrap();
            let hello = std::str::from_utf8(&plain).unwrap();
            assert_eq!(hello, hi);
        }
    }
}

#[test]
fn morus() {
    let client = Thrift {
        host: env::var("MORUS_HOST").unwrap(),
        port: env::var("MORUS_PORT").unwrap().parse().unwrap(),
        secure: false,
    };
    println!("connect to morus {}", client);
    {
        let res = client.to_html(
            r#"
# Hello, palm.

- line 1
- line 2

"#,
            true,
        );

        println!("MARKDOWN TO HTML: {:?}", res);
    }
}

#[test]
fn binary() {
    let hi = "hello, palm!";
    {
        let task = EmailSendTask {
            subject: hi.to_string(),
            body: Body {
                text: "<a href='https://www.google.com'>Google</a>".to_string(),
                html: true,
            },
            to: Address {
                name: "who-am-i".to_string(),
                email: "who-am-i@gmail.com".to_string(),
            },
            cc: BTreeSet::new(),
            bcc: BTreeSet::new(),
            attachments: BTreeSet::new(),
        };

        {
            let buf = to_bytes(&task).unwrap();
            println!("email task({}): {}", buf.len(), BASE64.encode(&buf));
            assert!(!buf.is_empty());

            let val: EmailSendTask = from_bytes(&buf).unwrap();
            println!("{:?}", val);
            assert_eq!(hi, val.subject);
        }
    }
}
