use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    // env_logger::builder().format_timestamp(None).init();
    env_logger::init();
    if let Err(e) = wisteria::app::run().await {
        log::error!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
