# nautobot-cli

[![crates.io](https://img.shields.io/crates/v/nautobot-cli.svg)](https://crates.io/crates/nautobot-cli)

a command-line interface (cli) for interacting with [nautobot](https://nautobot.com/), built using the `nautobot` rust crate.

## install

you can install the cli directly from crates.io (once published) or build it from source.

```bash
cargo install nautobot-cli
```

## configuration

the cli uses environment variables for configuration. you must set these before running commands:

```bash
export NAUTOBOT_URL=https://nautobot.example.com
export NAUTOBOT_TOKEN=your-api-token
```

## usage

the cli supports a variety of commands mapping to nautobot's api structure.

### general status

check the status of the nautobot instance:

```bash
nautobot-cli status
```

### resource management

most resources support `list`, `get`, `create`, `update`, and `delete` operations.

#### list resources

```bash
# list all devices
nautobot-cli dcim devices list

# list all sites
nautobot-cli dcim sites list
```

#### get a specific resource

retrieve a resource by its uuid:

```bash
nautobot-cli dcim devices get <device-uuid>
```

#### create a resource

create a new resource using a json payload:

```bash
nautobot-cli extras tags create --json '{"name": "production", "slug": "prod", "content_types": ["dcim.device"]}'
```

### help

to see all available commands and subcommands:

```bash
nautobot-cli --help
```