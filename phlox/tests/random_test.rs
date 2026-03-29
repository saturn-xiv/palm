#[test]
fn bytes() {
    let len = 16;
    for i in 1..=10 {
        let buf = phlox::random::bytes(16);
        println!("random bytes({}): {}", i, phlox::base64::encode(&buf));
        assert_eq!(len, buf.len());
    }
}

#[test]
fn alphanumeric() {
    let len = 16;
    for i in 1..=10 {
        let s = phlox::random::alphanumeric(16);
        println!("random alphanumeric string({}): {}", i, s);
        assert_eq!(len, s.len());
    }
}

#[test]
fn uuid() {
    for i in 1..=10 {
        let s = phlox::random::uuid();
        println!("uuid({}): {}", i, s);
    }
}
