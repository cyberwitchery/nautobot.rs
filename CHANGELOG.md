# changelog

## [unreleased]

### release
- add release SBOM generation and upload (CycloneDX)

## [0.2.0] - 2026-01-25

### features
- **config file support**: added `~/.config/nautobot-cli/config.toml` with named profiles
- **token sources**: three-tier token resolution (token_command > token_env > token)
- **config subcommand**: `nautobot-cli config {path,list,show,validate}` for profile management
- **`--profile` flag**: select configuration profile (default: "default")
- **`--columns` flag**: explicit column selection for table output (comma-separated)
- **`--max-columns` flag**: limit auto-selected columns in table output (default: 6)
- cli now at feature parity with netbox-cli for configuration and table output

### changes
- `--url` and `--token` are now optional when using config file or environment variables
- `--output` format can be set in config file profile

## [0.1.1] - 2026-01-23

### features
- exposed `Resource::new`, `Paginator::new`, and generic `Client` request methods (`get`, `post`, `put`, `patch`, `delete`) to allow generic/dynamic API adapters (like `alembic`).

## [0.1.0] - 2026-01-23

### documentation
- aligned documentation style with `netbox.rs` (lowercase, code-heavy)
- added comprehensive `README.md` for `nautobot` and `nautobot-cli` crates
- added functional examples in `crates/nautobot/examples/` (`status`, `raw_request`, `graphql_query`)
- updated cli documentation to reflect full resource coverage

### features
- full cli support for standard api operations

## [0.0.1] - 2026-01-22

### initial release
- initial project setup mirroring netbox.rs
- client library with comprehensive structure
- cli with initial set of commands
- openapi generation infrastructure

[unreleased]: https://github.com/cyberwitchery/nautobot.rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/cyberwitchery/nautobot.rs/releases/tag/v0.2.0
[0.1.1]: https://github.com/cyberwitchery/nautobot.rs/releases/tag/v0.1.1
[0.1.0]: https://github.com/cyberwitchery/nautobot.rs/releases/tag/v0.1.0
[0.0.1]: https://github.com/cyberwitchery/nautobot.rs/releases/tag/v0.0.1
