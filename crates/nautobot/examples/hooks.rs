//! demonstrates `HttpHooks` for request/response observation.
//!
//! run with:
//! ```bash
//! NAUTOBOT_URL=http://localhost:8000 NAUTOBOT_TOKEN=... cargo run -p nautobot --example hooks
//! ```

use nautobot::{Client, ClientConfig, HttpHooks};
use reqwest::{Method, Request, StatusCode};
use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

struct MetricsHook {
    request_count: Arc<AtomicU64>,
    total_ms: Arc<AtomicU64>,
}

impl HttpHooks for MetricsHook {
    fn on_request(
        &self,
        _method: &Method,
        _path: &str,
        request: &mut Request,
    ) -> nautobot::Result<()> {
        self.request_count.fetch_add(1, Ordering::Relaxed);
        request
            .headers_mut()
            .insert("x-client-id", "hooks-example".parse().expect("valid header"));
        Ok(())
    }

    fn on_response(
        &self,
        method: &Method,
        path: &str,
        status: StatusCode,
        duration: Duration,
    ) {
        let ms = duration.as_millis() as u64;
        self.total_ms.fetch_add(ms, Ordering::Relaxed);
        println!("{method} {path} -> {status} ({ms}ms)");
    }

    fn on_error(&self, method: &Method, path: &str, error: &nautobot::Error, duration: Duration) {
        let ms = duration.as_millis() as u64;
        eprintln!("{method} {path} -> ERROR after {ms}ms: {error}");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("NAUTOBOT_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let token = env::var("NAUTOBOT_TOKEN").expect("NAUTOBOT_TOKEN must be set");

    let request_count = Arc::new(AtomicU64::new(0));
    let total_ms = Arc::new(AtomicU64::new(0));

    let hook = MetricsHook {
        request_count: request_count.clone(),
        total_ms: total_ms.clone(),
    };

    let mut config = ClientConfig::new(url, token).with_http_hooks(hook);
    if env::var("NAUTOBOT_INSECURE").is_ok() {
        config = config.with_ssl_verification(false);
    }

    let client = Client::new(config)?;

    // make a couple of requests so the hooks fire
    let _ = client.status().status().await?;
    let _ = client.extras().tags().list(None).await?;

    println!(
        "\n{} requests, {}ms total",
        request_count.load(Ordering::Relaxed),
        total_ms.load(Ordering::Relaxed),
    );

    Ok(())
}
