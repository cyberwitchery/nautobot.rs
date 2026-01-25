use nautobot::{Client, ClientConfig};
use reqwest::Method;
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

    // Make a raw GET request to the status endpoint
    let response = client.request_raw(Method::GET, "status/", None).await?;

    println!("Raw status response:\n{:#?}", response);

    Ok(())
}
