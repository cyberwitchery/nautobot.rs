//! Local smoke tests against a live Nautobot instance.
//!
//! Run manually with:
//! NAUTOBOT_TOKEN=... NAUTOBOT_URL=http://localhost:8000 cargo test -p nautobot --test smoke_local -- --ignored

use nautobot::{Client, ClientConfig};

fn env_var(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn build_client() -> Option<Client> {
    let token = env_var("NAUTOBOT_TOKEN")?;
    let url = env_var("NAUTOBOT_URL").unwrap_or_else(|| "http://localhost:8000".to_string());
    let config = ClientConfig::new(url, token).with_max_retries(0);
    Client::new(config).ok()
}

#[tokio::test]
#[ignore]
async fn smoke_local_status() {
    let Some(client) = build_client() else {
        eprintln!("NAUTOBOT_TOKEN is not set; skipping smoke test");
        return;
    };

    let status = client.status().status().await.expect("status response");
    assert!(!status.is_empty(), "expected status payload");
}
