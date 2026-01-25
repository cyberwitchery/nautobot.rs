# nautobot-cli

cli for the nautobot api. covers status checks, resource operations, and a raw mode for any endpoint.

## install

from this repo:

```bash
cargo install --path crates/nautobot-cli
```

## quickstart

```bash
nautobot-cli --url https://nautobot.example.com --token $TOKEN status
```

## configuration

### config file

create `~/.config/nautobot-cli/config.toml` (or `~/Library/Application Support/nautobot-cli/config.toml` on macos):

```toml
[default]
url = "https://nautobot.example.com"
token_env = "NAUTOBOT_TOKEN"

[prod]
url = "https://nautobot.prod.example.com"
token_command = "pass show nautobot/prod-token"
timeout = 60
ssl_verify = true
output = "table"
```

### profile options

| option | description |
|--------|-------------|
| `url` | nautobot instance url |
| `token` | plain token (not recommended) |
| `token_env` | environment variable containing token |
| `token_command` | command to execute to get token |
| `timeout` | request timeout in seconds |
| `retries` | max retry attempts |
| `ssl_verify` | whether to verify ssl certificates |
| `output` | default output format (json, yaml, table) |

### token priority

token is resolved in this order:
1. `token_command` - execute shell command
2. `token_env` - read from environment variable
3. `token` - plain token string

### using profiles

```bash
# use default profile
nautobot-cli status

# use named profile
nautobot-cli --profile prod dcim devices list

# override profile settings
nautobot-cli --profile prod --url https://other.nautobot.com status
```

### config commands

```bash
# show config file path
nautobot-cli config path

# list available profiles
nautobot-cli config list

# show profile configuration
nautobot-cli --profile prod config show

# validate profile
nautobot-cli --profile prod config validate
```

## auth

via config file (recommended), or:

cli flags:
- `--url`
- `--token`

environment variables:
- `NAUTOBOT_URL`
- `NAUTOBOT_TOKEN`

priority: cli flags > environment variables > config file

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

get by id:

```bash
nautobot-cli dcim devices get <uuid>
```

create:

```bash
nautobot-cli extras tags create --json '{"name":"test","content_types":["dcim.device"]}'
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

### table output options

control which columns are shown:

```bash
# explicit columns
nautobot-cli --output table --columns id,name,status dcim devices list

# limit auto-selected columns (default: 6)
nautobot-cli --output table --max-columns 10 dcim devices list
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
nautobot-cli config --help
nautobot-cli dcim --help
```
