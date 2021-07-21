extern crate palm;

use std::ops::DerefMut;

use palm::cache::Provider;

#[test]
fn cache() {
    let ch = palm::cache::redis::Config::default();
    let ch = ch.open().unwrap();
    let mut ch = ch.get().unwrap();
    let ch = ch.deref_mut();
    println!("{}", ch.version().unwrap());
}
