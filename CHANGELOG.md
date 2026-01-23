# changelog

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

[unreleased]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/cyberwitchery/netbox.rs/releases/tag/v0.1.1
[0.1.0]: https://github.com/cyberwitchery/netbox.rs/releases/tag/v0.1.0
[0.0.1]: https://github.com/cyberwitchery/netbox.rs/releases/tag/v0.0.1
