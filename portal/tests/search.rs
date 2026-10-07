use hyacinth::lavender_v1;

#[test]
fn opensearch() {
    let cfg = portal::opensearch::Node::default();
    println!("{:?}", cfg);
    let cli = cfg.single().unwrap();
    println!("{}", cli.index_name::<lavender_v1::SystemdRequest>());
    println!("{}", cli.index_name::<lavender_v1::KubernetesRequest>());
    println!("{}", cli.index_name::<lavender_v1::http_request::Item>());
}
