use std::ops::DerefMut;

use chrono::Duration;
use fig::cache::{Provider, redis::Config};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Item {
    id: i32,
    name: String,
}

#[tokio::test]
async fn cache() {
    let cfg = Config::default();
    let pool = cfg.open().unwrap();

    {
        let mut ch = pool.get().unwrap();
        let ch = ch.deref_mut();

        for (k, v) in ch.version().unwrap() {
            println!("{} => {}", k, v);
        }
    }

    let len = 100;
    for i in 0..len {
        let mut ch = pool.get().unwrap();
        let ch = ch.deref_mut();

        let it = ch
            .get(
                &format!("test.{}", i),
                &|| {
                    Ok(Item {
                        id: i,
                        name: format!("hello, {}!", i),
                    })
                },
                Duration::seconds(1 << 12),
            )
            .unwrap();
        assert_eq!(it.id, i);
    }
    {
        let mut ch = pool.get().unwrap();
        let ch = ch.deref_mut();
        let keys = ch.keys().unwrap();
        println!("{} vs {}", keys.len(), len);
        // assert!(keys.len() >= len);
    }
}
