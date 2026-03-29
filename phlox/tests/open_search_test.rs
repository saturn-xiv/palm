use phlox::open_search::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    text: String,
}

#[tokio::test]
async fn info() {
    let cfg = Node {
        host: "http://localhost:9200".to_string(),
        namespace: Some("testing".to_string()),
    };

    let client = cfg.single().unwrap();
    {
        println!("index name(Message): {}", client.index_name::<Message>());
    }
    {
        let it = client.info().await.unwrap();
        println!("{:?}", it);
    }
}
