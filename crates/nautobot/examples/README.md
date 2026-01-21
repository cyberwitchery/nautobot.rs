# examples

all examples use these environment variables:

- `NAUTOBOT_URL` (default: `http://localhost:8000`)
- `NAUTOBOT_TOKEN` (required for auth)
- `NAUTOBOT_INSECURE` (optional, set to `1` to disable tls verification)

run an example:

```bash
cargo run -p nautobot --example status
```

```bash
cargo run -p nautobot --example raw_request
```
