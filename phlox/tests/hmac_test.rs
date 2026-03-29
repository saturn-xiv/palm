use phlox::{
    Mac, base64::encode as base64_encode, hmac::HmacSha512, random::bytes as random_bytes,
};

const HELLO: &str = "Hello, Phlox!";

#[test]
fn sha512() {
    let mac = HmacSha512::new(random_bytes(128));

    for i in 1..10 {
        let msg = format!("{}({})", HELLO, i);
        let buf = mac.sign(msg.as_bytes()).unwrap();
        println!("hmac-sha512({}): {}", msg, base64_encode(&buf));
        mac.verify(&buf, msg.as_bytes()).unwrap();
    }
}
