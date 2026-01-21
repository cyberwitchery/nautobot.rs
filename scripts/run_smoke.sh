#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${NAUTOBOT_TOKEN:-}" ]]; then
  echo "NAUTOBOT_TOKEN is required" >&2
  exit 1
fi

NAUTOBOT_URL="${NAUTOBOT_URL:-http://localhost:8000}"

echo "Running nautobot smoke tests against ${NAUTOBOT_URL}"

NAUTOBOT_URL="${NAUTOBOT_URL}" NAUTOBOT_TOKEN="${NAUTOBOT_TOKEN}" NAUTOBOT_INSECURE="${NAUTOBOT_INSECURE:-}" \
  cargo test -p nautobot --test smoke_local -- --ignored
