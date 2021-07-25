extern crate palm;

use palm::tty;

#[test]
fn ports() {
    for it in tty::ports().unwrap() {
        println!("find {}", it);
    }
}

#[test]
fn uuid() {
    println!("{it} {it:?}", it = tty::Uuid::null());
    println!("{it} {it:?}", it = tty::Uuid::zero());
    println!("{it} {it:?}", it = "abcd".parse::<tty::Uuid>().unwrap());
    for _ in 1..100 {
        println!("{it} {it:?}", it = tty::Uuid::default());
    }
}
