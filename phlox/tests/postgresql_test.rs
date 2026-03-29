use std::ops::DerefMut;

use phlox::orm::{Dao, postgresql::Node};

#[test]
fn ping() {
    let cfg = Node {
        host: "127.0.0.1".to_string(),
        port: 5432,
        user: "www".to_string(),
        password: Some("change-me".to_string()),
        db_name: "phlox_testing".to_string(),
        pool_size: 16,
    };
    let pool = cfg.open().unwrap();
    {
        let mut db = pool.get().unwrap();
        let db = db.deref_mut();
        {
            let it = Dao::heartbeat(db).unwrap();
            println!("{:?}", it);
        }
    }
}
