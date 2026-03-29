use phlox::ssha512;

const HELLO: &str = "Hello, Phlox!";

#[test]
fn ssha512() {
    for i in 1..10 {
        let hash = ssha512::sign(HELLO, 8).unwrap();
        println!("SSHA512({}): doveadm pw -t '{}' -p '{}'", i, hash, HELLO);
    }
}
