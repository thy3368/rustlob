#!/usr/bin/env bash
set -euo pipefail

: "${CHAIN:=Mainnet}"
: "${NODE_TYPE:=non-validator}"
: "${MAINNET_ROOT_IPS:=}"
: "${TESTNET_ROOT_IPS:=}"
: "${RESERVED_PEER_IPS:=}"
: "${EXTRA_FLAGS:=}"
[[ "$CHAIN" == Mainnet || "$CHAIN" == Testnet ]] || { echo "CHAIN must be Mainnet or Testnet" >&2; exit 2; }
[[ "$NODE_TYPE" == non-validator ]] || { echo "only NODE_TYPE=non-validator is supported" >&2; exit 2; }

root_ips="$MAINNET_ROOT_IPS"
[[ "$CHAIN" == Testnet ]] && root_ips="$TESTNET_ROOT_IPS"
csv_to_json() { jq -cn --arg value "$1" '$value | split(",") | map(gsub("^\\s+|\\s+$"; "")) | map(select(length > 0) | {Ip: .})'; }
mkdir -p "$HOME/hl/hyperliquid_data" "$HOME/hl/data" "$HOME/config"
jq -n --arg chain "$CHAIN" --argjson roots "$(csv_to_json "$root_ips")" \
  --argjson reserved "$(csv_to_json "$RESERVED_PEER_IPS")" \
  '{chain:$chain, root_node_ips:$roots, reserved_peer_ips:$reserved, try_new_peers:true}' > "$HOME/config/override_gossip_config.json"
cp "$HOME/config/override_gossip_config.json" "$HOME/override_gossip_config.json"

cd "$HOME"
exec /usr/local/bin/hl-visor run-non-validator \
  --serve-info --serve-eth-rpc --write-trades --write-fills --write-raw-book-diffs --write-order-statuses ${EXTRA_FLAGS}
