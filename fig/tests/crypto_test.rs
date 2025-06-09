use data_encoding::BASE64_NOPAD as BASE64;
use fig::crypto::{Aes, HMac, Password, Secret, random, ssha512};

#[test]
fn rand() {
    for _ in 0..3 {
        println!("random bytes: {}", BASE64.encode(&random::bytes(8)));
        println!("random string: {}", random::string(8));
        println!("random uuid: {}", random::uuid());
    }
}

#[test]
fn ssha512() {
    let salt = random::bytes(8);
    let plain = random::bytes(128);

    println!("ssha512 salt: {}", BASE64.encode(&salt));
    println!("ssha512 plain: {}", BASE64.encode(&plain));

    let cipher = ssha512::sum(&plain, &salt);
    println!("ssha512 cipher: {}", cipher);
    assert!(ssha512::verify(&cipher, &plain));
}

#[test]
fn hmac() {
    let key = random::bytes(24);
    let plain = random::bytes(128);

    let hmac = HMac::new(&BASE64.encode(&key)).unwrap();

    println!("hmac plain: {}", BASE64.encode(&plain));
    let cipher = hmac.sign(&plain).unwrap();
    println!("hmac cipher: {}", BASE64.encode(&cipher));
    assert!(hmac.verify(&cipher, &plain));
}

#[test]
fn aes() {
    let key = random::bytes(32);

    let aes = Aes::new(&BASE64.encode(&key)).unwrap();

    for plain in vec!["hi", "hello, aes!", "中文"] {
        for i in 1..5 {
            println!("######## {} ########", i);
            println!("aes plain: {:?}", plain);
            let (cipher, salt) = aes.encrypt(&plain.as_bytes()).unwrap();
            println!("aes cipher: {}", BASE64.encode(&cipher));
            println!("aes salt: {}", BASE64.encode(&salt));

            let value = aes.decrypt(&cipher, &salt).unwrap();
            let value = String::from_utf8(value).unwrap();
            println!("aes decode: {:?}", value);
            assert_eq!(plain, value);
        }
    }
}
