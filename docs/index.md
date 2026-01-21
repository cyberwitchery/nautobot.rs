# docs index

entrypoints:
- client guide: `crates/nautobot/docs/client.md`
- cli guide: `crates/nautobot-cli/docs/cli.md`
- examples: `crates/nautobot/examples/README.md`
- local nautobot: `docs/local-nautobot.md`

developer docs:
- dev guide: `docs/dev.md`
- codegen: `scripts/README.md`
- contributing: `CONTRIBUTING.md`

quick start:

```bash
NAUTOBOT_TOKEN=... cargo run -p nautobot --example status
NAUTOBOT_TOKEN=... cargo run -p nautobot --example graphql_query
```

for local nautobot setup, follow:
https://github.com/nautobot/nautobot-docker-compose
