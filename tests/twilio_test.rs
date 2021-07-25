extern crate palm;
extern crate tokio;

async fn sms() -> palm::errors::Result<()> {
    let cfg: palm::env::Config = palm::parser::from_toml("config.toml")?;
    let it = cfg.twilio.sms(env!("TO"), "Hello, palm!", None).await?;
    println!("{:?}", it);
    Ok(())
}

#[test]
fn test_inbound() {}

#[test]
fn test_sms() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        sms().await.unwrap();
    });
}
