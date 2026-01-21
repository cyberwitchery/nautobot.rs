# contributing

short, practical notes for working on this repo.

## local dev commands

```bash
cargo build
cargo test
cargo clippy --all-targets --all-features
cargo fmt --all
```

msrv: rust 1.85+

first builds on a new machine or fresh checkout are slow because the dependency cache is cold.

## docs build

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo doc --workspace --all-features --no-deps
```

open `target/doc/nautobot/index.html` for the main library docs.

## local smoke tests

manual smoke tests live under `crates/nautobot/tests/smoke_local.rs` and hit a live nautobot.

they are ignored by default. run with:

```bash
NAUTOBOT_TOKEN=... NAUTOBOT_URL=http://localhost:8000 cargo test -p nautobot --test smoke_local -- --ignored
```

## coverage

we use `cargo llvm-cov`.

install:

```bash
cargo install cargo-llvm-cov
```

run:

```bash
cargo llvm-cov --workspace --all-features --ignore-filename-regex 'crates/nautobot-openapi'
```

generate lcov (for ci or tooling):

```bash
cargo llvm-cov --workspace --all-features --ignore-filename-regex 'crates/nautobot-openapi' --lcov --output-path lcov.info
```

ci enforces a minimum line coverage of 75% while excluding generated code.

## documentation

- add rustdoc for public apis
- include examples for new features
- update `README.md` and `CHANGELOG.md` for user-visible changes

## api design principles

1. ergonomics
2. type safety
3. consistency
4. stability

## release checklist (maintainers)

1. update `CHANGELOG.md`
2. bump versions in `Cargo.toml`
3. bump `nautobot-openapi` in `[workspace.dependencies]`
4. regenerate openapi bindings
5. run tests and coverage
6. publish crates

## issue reporting

use https://github.com/cyberwitchery/nautobot.rs/issues

## local release

run:

```bash
./scripts/release_local.sh
```

set `SKIP_COVERAGE=1` to skip coverage.

notes:
- publish `nautobot-openapi` before `nautobot`
- the script uses `cargo package -p nautobot --no-verify` for local packaging

## release automation

we publish from tags matching `v*` (for example, `v0.1.0`).

the release workflow expects a repository secret named `CARGO_REGISTRY_TOKEN`.
