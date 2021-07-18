extern crate palm;

use std::ops::DerefMut;

use palm::cache::Provider;
use redis::cluster::ClusterClient;

#[test]
fn cache() {
    let ch = palm::cache::redis::Config::default();
    let ch = ch.open().unwrap();
    let mut ch = ch.get().unwrap();
    let ch = ch.deref_mut();
    // let mut ch = ClusterClient::get_connection(ch).unwrap();
    // let mut db = ch.get_connection::<ClusterClient>().unwrap();
    println!("{}", ch.version().unwrap());
}
