# roadmap

## 0.1.x (initial release)

- basic client structure
- cli with status and raw commands
- openapi generation setup

## 0.2.x (stabilization + cli ux)

- cli config profiles + token sources (env/command)
- integration harness + golden outputs for cli
- cli `--output table` column selection and max column count override

## 0.3.0 (library extensibility + observability)

- optional tracing instrumentation
- request middleware/hooks
- typed graphql response helper
- per-request retry/backoff tuning
