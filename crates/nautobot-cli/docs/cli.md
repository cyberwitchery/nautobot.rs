# nautobot-cli

cli for the nautobot api. currently covers status checks and a raw mode for any endpoint.

## install

from this repo:

```bash
cargo install --path crates/nautobot-cli
```

## quickstart

```bash
nautobot-cli --url https://nautobot.example.com --token $TOKEN status
```

## auth

required:
- `--url`
- `--token`

or set:
- `NAUTOBOT_URL`
- `NAUTOBOT_TOKEN`

## resources

list resource groups:

```bash
nautobot-cli resources
nautobot-cli resources dcim
```

## common commands

list:

```bash
nautobot-cli dcim devices list
nautobot-cli ipam prefixes list
```


## output formats

```bash
nautobot-cli status --output json
nautobot-cli status --output yaml
nautobot-cli status --output table
```

select a field with a dot path:

```bash
nautobot-cli status --select "ready"
```

## dry run

print the request for write operations without sending them:

```bash
nautobot-cli raw --method POST --path ipam/vrfs/ --json '{"name":"blue"}' --dry-run
```

## raw requests

use `raw` for any endpoint:

```bash
nautobot-cli raw --method GET --path dcim/devices/ --query "name=leaf-1" --query "limit=5"
nautobot-cli raw --method POST --path ipam/vrfs/ --json '{"name":"blue","rd":"65000:100"}'
```

notes:
- `--path` is api-relative, e.g. `dcim/devices/`
- a leading `api/` is stripped if present
- repeat `--query key=value` for multiple params

## help

```bash
nautobot-cli --help
```
