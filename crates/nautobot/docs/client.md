# client library

this crate provides a typed, ergonomic client for the nautobot rest api.

## install

```toml
[dependencies]
nautobot = "0.4"
tokio = { version = "1.0", features = ["full"] }
```

optional tracing instrumentation:

```toml
[dependencies]
nautobot = { version = "0.4", features = ["tracing"] }
tokio = { version = "1.0", features = ["full"] }
tracing-subscriber = "0.3"
```

tiny runtime setup example:

```rust,ignore
use nautobot::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("nautobot=trace")
        .init();

    let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
    let _ = client.status().status().await?;
    Ok(())
}
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

## http customization hooks

for cross-cutting behavior, you can inject a prebuilt `reqwest::Client`,
customize the internal client builder, and attach request/response hooks.

```rust,no_run
use nautobot::{Client, ClientConfig, HttpHooks};
use reqwest::{Method, Request, StatusCode};
use std::time::Duration;

struct MetricsHook;

impl HttpHooks for MetricsHook {
    fn on_request(&self, _method: &Method, _path: &str, request: &mut Request) -> nautobot::Result<()> {
        request
            .headers_mut()
            .insert("x-client-hook", "enabled".parse().expect("valid header value"));
        Ok(())
    }

    fn on_response(&self, method: &Method, path: &str, status: StatusCode, duration: Duration) {
        println!("{method} {path} -> {status} in {duration:?}");
    }
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let prebuilt = reqwest::Client::builder()
        .pool_max_idle_per_host(4)
        .build()?;

    let config = ClientConfig::new("https://nautobot.example.com", "token")
        .with_http_client(prebuilt)
        .with_http_hooks(MetricsHook);

    let _client = Client::new(config)?;
    Ok(())
}
```

precedence and behavior:
- `with_http_client(...)` takes precedence over `with_http_client_builder(...)`.
- hooks run for all client-driven HTTP requests (`Resource<T>`, special endpoint helpers, and `request_raw`).
- hooks are additive with tracing; they can be used independently or together.

safety notes:
- avoid logging or exporting authentication tokens, cookies, or full sensitive payloads from hooks.
- prefer additive mutations (headers/metadata) over replacing request method/url/body unexpectedly.
- keep hook logic lightweight; slow hooks directly add latency to every request.

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

// get a device by UUID
let device = client.dcim().devices().get("device-uuid-here").await?;
println!("Device: {}", device.display.as_deref().unwrap_or("<unknown>"));

// create a tag (note: content_types is required in Nautobot)
use nautobot::models::TagRequest;
let tag = TagRequest::new(vec!["dcim.device".to_string()], "my-tag".to_string());
let created = client.extras().tags().create(&tag).await?;

// delete by UUID (id is a UUID)
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
// example of raw request error handling
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

// access raw JSON API
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
use nautobot::models::ApprovalWorkflowStageResponseApprovalWorkflowStage;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;

// list available IPs in a prefix
let available = client.ipam().prefix_available_ips("prefix-uuid", None).await?;
println!("Available IPs: {}", available.count);

// allocate an IP from a prefix
let status = ApprovalWorkflowStageResponseApprovalWorkflowStage::new();
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

// trace an interface path
let trace = client.dcim().interface_trace("interface-uuid").await?;

// trace a power port
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

// run a job by UUID
let response = client.extras().job_run("job-uuid", &JobInputRequest::new()).await?;

// run a job by name
let response = client.extras().job_run_by_name("my-job", &JobInputRequest::new()).await?;

// approve a scheduled job
let approved = client.extras().scheduled_job_approve("scheduled-job-uuid").await?;
# Ok(())
# }
```
