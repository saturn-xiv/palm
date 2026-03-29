use phlox::orm::{Dao, sqlite3::Node};

#[test]
fn ping() {
    let cfg = Node {
        file: "/tmp/testing.sqlite3".to_string(),
    };
    let mut db = cfg.open().unwrap();

    {
        let it = Dao::heartbeat(&mut db).unwrap();
        println!("{:?}", it);
    }
}
