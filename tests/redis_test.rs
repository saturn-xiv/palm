extern crate palm;

#[test]
fn cache() {
    let ch = palm::cache::redis::Config {
        prefix: Some("test://".to_string()),
        ..Default::default()
    }
    .open()
    .unwrap();
    let ch = ch.get().unwrap();
}
