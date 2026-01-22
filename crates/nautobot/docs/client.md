# client library

this crate provides a typed, ergonomic client for the nautobot rest api.

## install

```toml
[dependencies]
nautobot = "0.1.0"
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

## create, update, delete

nautobot uses uuid strings for all resource ids, not integers.

```rust,no_run
use nautobot::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;

// Get a device by UUID
let device = client.dcim().devices().get("device-uuid-here").await?;
println!("Device: {}", device.display.as_deref().unwrap_or("<unknown>"));

// Create a tag (note: content_types is required in Nautobot)
use nautobot::models::TagRequest;
let tag = TagRequest::new(vec!["dcim.device".to_string()], "my-tag".to_string());
let created = client.extras().tags().create(&tag).await?;

// Delete by UUID (id is a UUID)
let tag_id = created.id.expect("tag should have id").to_string();
client.extras().tags().delete(&tag_id).await?;
# Ok(())
# }
```

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

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;

// Access raw JSON API
let value = client.request_raw(reqwest::Method::GET, "dcim/devices/", None).await?;
println!("{}", value);
# Ok(())
# }
```

## special endpoints

nautobot provides ergonomic helpers for special endpoints beyond basic CRUD:

### ipam allocation

```rust,no_run
use nautobot::{Client, ClientConfig};
use nautobot::ipam::{IpAllocationRequest, PrefixLengthRequest};
use nautobot::models::BulkWritableCableRequestStatus;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;

// List available IPs in a prefix
let available = client.ipam().prefix_available_ips("prefix-uuid", None).await?;
println!("Available IPs: {}", available.count);

// Allocate an IP from a prefix
let status = BulkWritableCableRequestStatus::new();
let request = IpAllocationRequest::new(status);
let allocated = client.ipam().allocate_prefix_ips("prefix-uuid", &[request]).await?;
# Ok(())
# }
```

### dcim tracing

```rust,no_run
use nautobot::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;

// Trace an interface path
let trace = client.dcim().interface_trace("interface-uuid").await?;

// Trace a power port
let power_trace = client.dcim().power_port_trace("power-port-uuid").await?;
# Ok(())
# }
```

### extras jobs

```rust,no_run
use nautobot::{Client, ClientConfig};
use nautobot::extras::JobInputRequest;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;

// Run a job by UUID
let response = client.extras().job_run("job-uuid", &JobInputRequest::new()).await?;

// Run a job by name
let response = client.extras().job_run_by_name("my-job", &JobInputRequest::new()).await?;

// Approve a scheduled job
let approved = client.extras().scheduled_job_approve("scheduled-job-uuid").await?;
# Ok(())
# }
```
