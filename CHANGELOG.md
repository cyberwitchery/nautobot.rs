# changelog

## Unreleased

### fixed
- `openapi_config` now sets `User-Agent` and `Content-Type` default headers on its HTTP client (previously it only forwarded `extra_headers`). `Authorization` is intentionally left out of the defaults, since the generated code adds it per request, avoiding a duplicate header.
- `LimitedPaginator::collect_all` no longer over-allocates: it caps capacity at `max_pages * page_size` instead of the total count across all pages.

### compatibility
- tested and generated against Nautobot 3.1.3 (no API changes from 3.1.2).

## [0.4.1] - 2026-05-31

### fixed
- `docs.rs` builds now succeed: the per-tag OpenAPI API modules (`dcim_api`, `ipam_api`, `vpn_api`, etc.) are gated out of docs builds. The high-level `nautobot` client doesn't use them, so crates.io users see no change; only the per-tag pages on `docs.rs/nautobot-openapi/` are dropped.

### openapi
- regenerated bindings from Nautobot 3.1.2: `image_height`/`image_width` removed from image-attachment request bodies, `current_head` removed from git-repository request bodies.

### compatibility
- tested against Nautobot 3.1.1 and 3.1.2.

## [0.4.0] - 2026-04-26

### changed
- MSRV raised from 1.85 to 1.91.

### cli
- print a warning to stderr when the config file exists but can't be parsed, instead of silently ignoring it.
- fix a panic in `compact_json` on multi-byte UTF-8 (truncation now respects char boundaries).

### openapi
- regenerated bindings from Nautobot 3.1.0: new models for approval workflows, data validation, load balancers, and VPN; removed `BulkWritableCableRequestStatus` and `ScheduledJobApprovedByUser`.

### compatibility
- tested against Nautobot 3.1.0.

## [0.3.1] - 2026-02-21

### client
- add `DcimApi::location_stats()` to retrieve statistics for a location and its descendants.

### cli
- fix table output: render explicit `--columns` headers even when the result set is empty.

### openapi
- regenerated bindings from Nautobot 2.4.27: new models (`Stats`, `PaginatedStatsList`, `DuplexEnum`, `InterfaceDuplex`, `BulkWritableInterfaceTemplateRequestDuplex`) and endpoints (`dcim/locations/{id}/stats/`, scheduled-job delete operations).

### docs & examples
- add `examples/hooks.rs` demonstrating `HttpHooks` (header injection, response timing, error reporting).
- add a compatibility matrix (`docs/compat.md`) mapping client releases to tested Nautobot versions.

### compatibility
- tested against Nautobot 2.4.27.

## [0.3.0] - 2026-02-20

### client
- add configurable HTTP extension points: inject a prebuilt `reqwest` client, customize the client builder, or attach request/response hooks (`HttpHooks`).
- add an optional `tracing` feature for request-lifecycle instrumentation (URL build, send/response timing, retries, error classification).
- add `GraphqlApi::query_typed::<T>()` to deserialize the GraphQL `data` field into a typed struct directly.

### release
- releases now ship a CycloneDX SBOM.

## [0.2.0] - 2026-01-25

### features
- config file support: `~/.config/nautobot-cli/config.toml` with named profiles.
- three-tier token resolution (`token_command` > `token_env` > `token`).
- `config` subcommand (`path`, `list`, `show`, `validate`) for profile management.
- `--profile` flag to select a configuration profile (default `default`).
- `--columns` flag for explicit table columns, and `--max-columns` to limit auto-selected columns (default 6).
- the CLI is now at feature parity with `netbox-cli` for configuration and table output.

### changed
- `--url` and `--token` are now optional when a config file or environment variables are present.
- the `--output` format can be set per profile in the config file.

## [0.1.1] - 2026-01-23

### client
- expose `Resource::new`, `Paginator::new`, and the generic `Client` request methods (`get`, `post`, `put`, `patch`, `delete`) so generic/dynamic API adapters (such as alembic) can be built on top.

## [0.1.0] - 2026-01-23

### features
- full CLI support for standard API operations.

### documentation
- comprehensive `README.md` for the `nautobot` and `nautobot-cli` crates.
- functional examples in `crates/nautobot/examples/` (`status`, `raw_request`, `graphql_query`).

## [0.0.1] - 2026-01-22

### initial release
- client library, CLI, and OpenAPI generation infrastructure, mirroring netbox.rs.

[0.4.0]: https://github.com/cyberwitchery/nautobot.rs/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/cyberwitchery/nautobot.rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/cyberwitchery/nautobot.rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/cyberwitchery/nautobot.rs/releases/tag/v0.2.0
[0.1.1]: https://github.com/cyberwitchery/nautobot.rs/releases/tag/v0.1.1
[0.1.0]: https://github.com/cyberwitchery/nautobot.rs/releases/tag/v0.1.0
[0.0.1]: https://github.com/cyberwitchery/nautobot.rs/releases/tag/v0.0.1
