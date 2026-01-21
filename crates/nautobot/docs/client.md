# client library

this crate provides a typed, ergonomic client for the nautobot rest api.

## install

```toml
[dependencies]
nautobot = { path = "../nautobot" } # Use version 0.1.0 when published
tokio = { version = "1.0", features = ["full"] }
```

## create a client

```rust,no_run
use nautobot::{Client, ClientConfig};

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("https://nautobot.example.com", "token");
    let _client = Client::new(config)?;
    Ok(())
}
```

## auth and config

```rust,no_run
use std::time::Duration;
use nautobot::ClientConfig;
use reqwest::header::{HeaderName, HeaderValue};

fn example() {
    let _config = ClientConfig::new("https://nautobot.example.com", "token")
        .with_timeout(Duration::from_secs(60))
        .with_max_retries(5)
        .with_ssl_verification(false)
        .with_header(
            HeaderName::from_static("x-nautobot-client"),
            HeaderValue::from_static("custom"),
        );
}
```

## http client access

```rust,no_run
use nautobot::{Client, ClientConfig};

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
    let http = client.http_client();
    let _ = http;
    Ok(())
}
```

<!--
## list and filter

```rust,no_run
use nautobot::{Client, ClientConfig, QueryBuilder};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
let query = QueryBuilder::new()
    .filter("status", "active")
    .limit(50)
    .order_by("name");

let page = client.dcim().devices().list(Some(query)).await?;
println!("{}", page.count);
# Ok(())
# }
```

## paginate

```rust,no_run
use nautobot::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
let mut paginator = client.dcim().devices().paginate(None)?;
while let Some(page) = paginator.next_page().await? {
    for device in page.results {
        let display = device.display.as_deref().unwrap_or("<unknown>");
        println!("{display}");
    }
}
# Ok(())
# }
```

note: for local dev instances with self-signed certs, call `with_ssl_verification(false)` in your config.
note: `paginate` returns `Result<Paginator<T>>`. handle errors before calling `next_page`.

## create, update, delete

```rust,no_run
use nautobot::{Client, ClientConfig};
// use nautobot::dcim::CreateDeviceRequest;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
// Implementation coming soon
# Ok(())
# }
```
-->

## error handling

```rust,no_run
use nautobot::{Client, ClientConfig, Error};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
// Example of raw request error handling
match client.request_raw(reqwest::Method::GET, "invalid/", None).await {
    Ok(_) => println!("success"),
    Err(Error::ApiError { status, .. }) if status == 404 => {
        println!("not found");
    }
    Err(e) if e.is_auth_error() => {
        println!("auth failed: {}", e);
    }
    Err(e) => {
        println!("error: {}", e);
    }
}
# Ok(())
# }
```

## raw api access

use this when you need an endpoint not yet wrapped by the high-level client.

```rust,no_run
use nautobot::{Client, ClientConfig};
// use nautobot::openapi::apis::dcim_api;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
let openapi_config = client.openapi_config()?;
// let device = dcim_api::dcim_devices_retrieve(&openapi_config, 42).await?;
// println!("{}", device.display.as_deref().unwrap_or("<unknown>"));
# Ok(())
# }
```
