use phlox::minio::{Client, Config};

#[test]
fn object_name() {
    for it in vec!["a.png", "b.txt", "c.mp3"] {
        println!("{} => {}", it, Client::object(it));
    }
}

#[tokio::test]
async fn buckets() {
    let config_file = std::env::var("MINIO_CONFIG").unwrap();
    let client = Config::open(config_file, Some("testing")).unwrap();
    for it in client.list_buckets().await.unwrap().iter() {
        println!("found bucket {}", it);
    }
}
