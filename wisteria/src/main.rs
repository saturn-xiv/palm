use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    // env_logger::builder().format_timestamp(None).init();
    env_logger::init();

    portal::graphql::site::status::Item::launched_at();

    if let Err(e) = wisteria::app::run().await {
        log::error!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
