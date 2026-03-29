use std::default::Default;

use chrono::Duration;
use phlox::{
    Result,
    cache::redis::{Client, Commands, FlexBuffersMessage, ManageConnection, Node, RedisError},
};
use serde::{Deserialize, Serialize};

const HI: &str = "hi";
const HELLO: &str = "Hello, Palm!";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    text: String,
}

#[test]
fn single() {
    let cfg = Node::default();
    let cli = cfg.single().unwrap();
    execute(&cli).unwrap();
}
#[test]
fn cluster() {
    let cfg = Node {
        host: "127.0.0.1".to_string(),
        port: 6371,
        pool_size: 8,
        namespace: Some("testing".to_string()),
    };
    let cli = cfg.cluster().unwrap();
    execute(&cli).unwrap();
}

fn execute<C: Commands, T: ManageConnection<Connection = C, Error = RedisError>>(
    client: &Client<C, T>,
) -> Result<()> {
    let it = Message {
        text: HELLO.to_string(),
    };
    {
        let key = format!("{}.0", HI);
        client.set(&key, &it, None)?;
        {
            let tmp: Message = client.get(key)?;
            assert_eq!(tmp.text, it.text);
        }
    }

    {
        let key = format!("{}.1", HI);
        client.set(&key, &it, Some(Duration::hours(1).to_std()?))?;
        {
            let tmp: Message = client.get(key)?;
            assert_eq!(tmp.text, it.text);
        }
    }
    Ok(())
}
