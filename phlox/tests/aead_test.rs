use phlox::{
    Enigma, aead::Aes256GcmSiv, base64::encode as base64_encode, random::bytes as random_bytes,
};

const HELLO: &str = "Hello, Phlox!";

#[test]
fn aes256_gcm_siv() {
    let key = random_bytes(32);
    println!("key: {}", base64_encode(&key));
    let aead = Aes256GcmSiv::new(&key).unwrap();
    for i in 1..10 {
        let (nonce, code) = aead.encrypt(HELLO.as_bytes()).unwrap();
        println!(
            "aes256-gcm-siv({}): ({}, {}) {}",
            i,
            HELLO,
            base64_encode(&nonce),
            base64_encode(&code)
        );
        {
            let it = aead.decrypt(&code, &nonce).unwrap();
            assert_eq!(it, HELLO.as_bytes());
        }
    }
}
