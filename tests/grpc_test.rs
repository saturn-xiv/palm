use palm::plugins::{
    nut::services::Session,
    pi::services::{os::Service as OsService, os_client::OsClient, os_server::OsServer},
};
use tonic::transport::Server;

#[tokio::test]
async fn server() {
    env_logger::init();
    let addr = "0.0.0.0:9999".parse().unwrap();

    println!("OsServer listening on {}", addr);

    Server::builder()
        .add_service(OsServer::new(OsService {}))
        .serve(addr)
        .await
        .unwrap();
}

#[tokio::test]
async fn client() {
    let mut cli = OsClient::connect("http://127.0.0.1:9999").await.unwrap();
    let mut req = tonic::Request::new(());
    Session::auth(&mut req, "token").unwrap();
    let res = cli.uptime(req).await.unwrap();
    println!("RESPONSE={:?}", res);
}
