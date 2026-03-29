use phlox::{Enigma, aead::Aes256GcmSiv, base64::encode as base64_encode, random};

const HELLO: &str = "Hello, Phlox!";

#[test]
fn aes256_gcm_siv() {
    let aead = Aes256GcmSiv::new(&random::bytes(32)).unwrap();
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
