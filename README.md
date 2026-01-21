# nautobot.rs

An API and ergonomic client for Nautobot, inspired by and structured similarly to `netbox.rs`.

## Structure

- `crates/nautobot-openapi`: Low-level generated bindings (placeholder).
- `crates/nautobot`: Ergonomic Rust client.
- `crates/nautobot-cli`: CLI tool for interacting with Nautobot.

## Usage

### Library

Add to your `Cargo.toml`:

```toml
[dependencies]
nautobot = { path = "path/to/nautobot.rs/crates/nautobot" }
```

```rust
use nautobot::{Client, ClientConfig};

#[tokio::main]
async fn main() {
    let config = ClientConfig::new("https://nautobot.example.com", "your-token");
    let client = Client::new(config).unwrap();
    
    // Use client...
}
```

### Helpers

```rust
# use nautobot::{Client, ClientConfig};
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
# let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
let devices = client.dcim().devices();
let notes = devices.notes(123, None).await?;
println!("notes: {}", notes.count);

let available_ips = client.ipam().prefix_available_ips("42", None).await?;
println!("available IPs: {}", available_ips.count);
# Ok(())
# }
```

### CLI

```bash
cargo run -p nautobot-cli -- --url https://nautobot.example.com --token your-token status
```
