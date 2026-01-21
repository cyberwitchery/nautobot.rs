#!/usr/bin/env bash
# Fetch OpenAPI schema from a running Nautobot instance
#
# Usage:
#   ./scripts/fetch_schema.sh [NAUTOBOT_URL]
#
# Environment variables:
#   NAUTOBOT_URL: URL of the Nautobot instance (default: http://localhost:8080)
#   NAUTOBOT_TOKEN: Optional API token for authenticated access

set -euo pipefail

NAUTOBOT_URL="${1:-${NAUTOBOT_URL:-http://localhost:8080}}"
SCHEMA_FILE="scripts/openapi-schema.json"

echo "Fetching OpenAPI schema from ${NAUTOBOT_URL}/api/docs/..."

# Construct curl command with optional auth header
CURL_CMD="curl -fsSL"
if [ -n "${NAUTOBOT_TOKEN:-}" ]; then
    CURL_CMD="$CURL_CMD -H 'Authorization: Token ${NAUTOBOT_TOKEN}'"
fi

# Fetch and pretty-print the schema
eval "$CURL_CMD '${NAUTOBOT_URL}/api/docs/?format=openapi'" | \
    jq '.' > "${SCHEMA_FILE}"

echo "Schema saved to ${SCHEMA_FILE}"
echo "Schema info:"
jq -r '.info | "  Version: \(.version)\n  Title: \(.title)"' "${SCHEMA_FILE}"
