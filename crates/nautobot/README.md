# nautobot

[![crates.io](https://img.shields.io/crates/v/nautobot.svg)](https://crates.io/crates/nautobot)
[![docs.rs](https://docs.rs/nautobot/badge.svg)](https://docs.rs/nautobot)

ergonomic rust client for the [nautobot](https://nautobot.com/) rest api.

this crate provides a high-level, typed interface for interacting with nautobot, built on top of generated openapi bindings.

## install

add this to your `Cargo.toml`:

```toml
[dependencies]
nautobot = "0.1.1"
tokio = { version = "1.0", features = ["full"] }
```

## usage

### basic example

```rust
use nautobot::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize the client
    let config = ClientConfig::new("https://nautobot.example.com", "your-api-token");
    let client = Client::new(config)?;

    // list devices from dcim
    let devices = client.dcim().devices().list(None).await?;
    println!("found {} devices", devices.count);

    // iterate through results
    for device in devices.results {
        println!("device: {}", device.name.unwrap_or_default());
    }

    Ok(())
}
```

### ergonomic helpers

the client includes helper methods for common operations that go beyond basic crud:

```rust
// ipam: find available ips in a prefix
let available_ips = client.ipam().prefix_available_ips("prefix-uuid", None).await?;

// dcim: trace a cable path from an interface
let trace = client.dcim().interface_trace("interface-uuid").await?;

// extras: run a job
use nautobot::extras::JobInputRequest;
let response = client.extras().job_run("job-uuid", &JobInputRequest::new()).await?;
```

## features

- **typed apis**: specific helpers for dcim, ipam, extras, virtualization, and more.
- **pagination**: built-in support for handling paginated responses.
- **async**: built on `tokio` and `reqwest` for non-blocking i/o.
- **openapi**: leverages `nautobot-openapi` for comprehensive schema coverage.
