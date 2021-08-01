use palm::app::launch;

#[actix_web::main]
async fn main() {
    env_logger::init();
    if let Err(e) = launch().await {
        log::error!("{:?}", e);
    }
}
