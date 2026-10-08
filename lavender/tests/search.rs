use portal::{graphql::Page, opensearch::Node as OpenSearch};

#[tokio::test]
async fn logging() {
    let node = OpenSearch {
        namespace: Some("wisteria.dev.261007".to_string()),
        ..Default::default()
    };
    let client = node.single().unwrap();
    {
        let url = "https://ifconfig.me/all.json";
        let (count, query) = lavender::graphql::monitoring::http::Item::queries_by_url(url);
        let (items, pagination) = client
            .pagination::<lavender::graphql::monitoring::http::Item>(
                count,
                query,
                &Page { index: 1, size: 60 },
            )
            .await
            .unwrap();

        println!("{:?}", pagination);
        for it in items.iter() {
            println!("{:?}", it);
        }
    }
}
