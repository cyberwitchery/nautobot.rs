use nautobot::{Client, ClientConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("NAUTOBOT_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let token = env::var("NAUTOBOT_TOKEN").expect("NAUTOBOT_TOKEN must be set");

    let mut config = ClientConfig::new(url, token);
    if env::var("NAUTOBOT_INSECURE").is_ok() {
        config = config.with_ssl_verification(false);
    }

    let client = Client::new(config)?;
    let status = client.status().status().await?;

    println!("{:#?}", status);

    Ok(())
}
