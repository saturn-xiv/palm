use data_encoding::BASE64_NOPAD;

#[test]
fn random_bytes() {
    let len = 32;
    for i in 1..=9 {
        let buf = portal::random::bytes(len);
        assert_eq!(buf.len(), len);
        println!("random bytes({}): {}", i, BASE64_NOPAD.encode(&buf));
    }
}

#[test]
fn random_alphanumeric() {
    let len = 32;
    for i in 1..=9 {
        let buf = portal::random::alphanumeric(len);
        assert_eq!(buf.len(), len);
        println!("random alphanumeric({}): {}", i, buf);
    }
}

#[test]
fn ssha512() {
    let password = "Hello, Palm!";
    let len = 8;
    for i in 1..=9 {
        let code = portal::ssha512::sign(password, len);
        assert!(portal::ssha512::verify(&code, password));
        println!("SSHA512({}): doveadm pw -t '{}' -p '{}'", i, code, password);
    }
}
