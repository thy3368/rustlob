#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
[[ -f .env ]] || { echo '.env is missing; run ./run_node.sh first' >&2; exit 1; }
docker compose --env-file .env ps
docker compose --env-file .env logs --tail=40 consensus
du -sh runtime/hl/hyperliquid_data runtime/hl/data 2>/dev/null || true
curl --fail --silent --show-error -H 'Content-Type: application/json' \
  --data '{"type":"meta"}' "http://127.0.0.1:${INFO_PORT:-3000}/info" || true
curl --fail --silent --show-error -H 'Content-Type: application/json' \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  "http://127.0.0.1:${EVM_RPC_PORT:-3001}/evm" || true
