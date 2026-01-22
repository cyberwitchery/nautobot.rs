# nautobot.rs

An API and ergonomic Rust client for [Nautobot](https://nautobot.com/), inspired by and structured similarly to `netbox.rs`.

## Structure

- `crates/nautobot-openapi`: Low-level generated bindings from the Nautobot OpenAPI spec.
- `crates/nautobot`: Ergonomic Rust client with typed helpers for all major endpoints.
- `crates/nautobot-cli`: CLI tool for interacting with Nautobot.

## Usage

### Library

Add to your `Cargo.toml`:

```toml
[dependencies]
nautobot = { path = "path/to/nautobot.rs/crates/nautobot" }
tokio = { version = "1.0", features = ["full"] }
```

```rust
use nautobot::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("https://nautobot.example.com", "your-token");
    let client = Client::new(config)?;

    // List devices
    let devices = client.dcim().devices().list(None).await?;
    println!("Found {} devices", devices.count);

    Ok(())
}
```

### Ergonomic Helpers

The client provides ergonomic helpers for common operations beyond basic CRUD:

```rust
# use nautobot::{Client, ClientConfig};
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
# let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
// IPAM: List available IPs in a prefix
let available_ips = client.ipam().prefix_available_ips("prefix-uuid", None).await?;
println!("Available IPs: {}", available_ips.count);

// DCIM: Trace an interface path
let trace = client.dcim().interface_trace("interface-uuid").await?;
println!("Traced interface: {}", trace.name);

// Extras: Run a job
use nautobot::extras::JobInputRequest;
let response = client.extras().job_run("job-uuid", &JobInputRequest::new()).await?;
# Ok(())
# }
```

### CLI

```bash
# Set environment variables
export NAUTOBOT_URL=https://nautobot.example.com
export NAUTOBOT_TOKEN=your-token

# Check status
cargo run -p nautobot-cli -- status

# List devices
cargo run -p nautobot-cli -- dcim devices list

# Get a specific device by UUID
cargo run -p nautobot-cli -- dcim devices get <uuid>

# Create a resource from JSON
cargo run -p nautobot-cli -- extras tags create --json '{"name":"test","slug":"test","content_types":["dcim.device"]}'
```

## Features

- **Full API coverage** via generated OpenAPI bindings
- **Ergonomic typed helpers** for DCIM, IPAM, Extras, Circuits, Cloud, and more
- **Special endpoints**: trace, available-ips, job execution, git sync, etc.
- **Pagination support** with async iterator
- **Configurable retry logic** and timeout handling
- **CLI tool** for quick API interactions
