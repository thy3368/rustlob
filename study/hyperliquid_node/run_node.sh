#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
if [[ ! -e .env ]]; then cp default.env .env; echo "Initialized .env from default.env; review seed peers before syncing."; fi
mkdir -p runtime/hl/hyperliquid_data runtime/hl/data runtime/config
docker compose --env-file .env up -d --build
