# local nautobot quickstart

this repo does not run nautobot for you. use the official nautobot-docker setup.

## setup

follow the instructions at:
https://github.com/nautobot/nautobot-docker-compose

## confirm it is running

ensure `http://localhost:8000` responds and you have a token.
(Note: Nautobot default port might differ, e.g., 8080. Adjust accordingly.)

## smoke test

run the local smoke test against your instance:

```bash
NAUTOBOT_TOKEN=... NAUTOBOT_URL=http://localhost:8000 cargo test -p nautobot --test smoke_local -- --ignored
```
