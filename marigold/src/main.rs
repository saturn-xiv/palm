#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(e) = marigold::app::run().await {
        log::info!("{}", e);
    }
}
