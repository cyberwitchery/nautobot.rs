# nautobot.rs

rust client for the nautobot rest api.

## structure

- `crates/nautobot-openapi`: low-level generated bindings from the nautobot openapi spec.
- `crates/nautobot`: ergonomic rust client with typed helpers for all major endpoints.
- `crates/nautobot-cli`: cli tool for interacting with nautobot.

## install

add to `Cargo.toml`:

```toml
[dependencies]
nautobot = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## quick start

```rust
use nautobot::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("https://nautobot.example.com", "your-token");
    let client = Client::new(config)?;

    // list devices
    let devices = client.dcim().devices().list(None).await?;
    println!("found {} devices", devices.count);

    Ok(())
}
```

## examples

### ergonomic helpers

```rust
# use nautobot::{Client, ClientConfig};
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
# let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
// ipam: list available ips in a prefix
let available_ips = client.ipam().prefix_available_ips("prefix-uuid", None).await?;
println!("available ips: {}", available_ips.count);

// dcim: trace an interface path
let trace = client.dcim().interface_trace("interface-uuid").await?;
println!("traced interface: {}", trace.name);

// extras: run a job
use nautobot::extras::JobInputRequest;
let response = client.extras().job_run("job-uuid", &JobInputRequest::new()).await?;
# Ok(())
# }
```

### cli

```bash
# set environment variables
export NAUTOBOT_URL=https://nautobot.example.com
export NAUTOBOT_TOKEN=your-token

# check status
cargo run -p nautobot-cli -- status

# list devices
cargo run -p nautobot-cli -- dcim devices list

# get a specific device by uuid
cargo run -p nautobot-cli -- dcim devices get <uuid>

# create a resource from json
cargo run -p nautobot-cli -- extras tags create --json '{"name":"test","slug":"test","content_types":["dcim.device"]}'
```

## features

- **full api coverage** via generated openapi bindings
- **ergonomic typed helpers** for dcim, ipam, extras, circuits, cloud, and more
- **special endpoints**: trace, available-ips, job execution, git sync, etc.
- **pagination support** with async iterator
- **configurable retry logic** and timeout handling
- **cli tool** for quick api interactions