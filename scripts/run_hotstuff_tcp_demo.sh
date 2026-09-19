#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN="target/debug/hotstuff_tcp_main"
LEADER_URL="${HOTSTUFF_DEMO_LEADER_URL:-http://127.0.0.1:39001/spot-block/commands}"
INTERVAL_SECONDS="${HOTSTUFF_DEMO_COMMAND_INTERVAL_SECONDS:-1}"
LOG_DIR="${HOTSTUFF_DEMO_LOG_DIR:-/tmp/hotstuff_use_case_demo}"

mkdir -p "$LOG_DIR"

cargo build -p hotstuff_use_case_demo --bin hotstuff_tcp_main

pids=()

cleanup() {
    local pid
    for pid in "${pids[@]:-}"; do
        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null || true
        fi
    done
    wait "${pids[@]:-}" 2>/dev/null || true
}

trap cleanup EXIT INT TERM

start_node() {
    local node_index="$1"
    local log_file="$LOG_DIR/node${node_index}.log"

    echo "[hotstuff_tcp_demo] starting node${node_index}, log=${log_file}"
    HOTSTUFF_DEMO_NODE_INDEX="$node_index" "$BIN" >"$log_file" 2>&1 &
    pids+=("$!")
}

wait_for_leader_http() {
    local attempt

    for attempt in $(seq 1 30); do
        if curl -sS --max-time 1 -o /dev/null "$LEADER_URL"; then
            echo "[hotstuff_tcp_demo] leader HTTP ready: ${LEADER_URL}"
            return 0
        fi
        sleep 1
    done

    echo "[hotstuff_tcp_demo] leader HTTP not ready after 30s: ${LEADER_URL}" >&2
    return 1
}

send_command() {
    local seq_no="$1"
    local now
    local order_id
    local cloid
    local body
    local status

    now="$(date +%s)"
    order_id="tcp-script-buy-${now}-${seq_no}"
    cloid="tcp-script-cloid-${now}-${seq_no}"
    body="$(printf '{"PlaceMatch":{"Single":{"party_id":"buyer","asset":10001,"order_id":"%s","symbol":"BTCUSDT","is_buy":true,"price":"100","size":"2","order_type":{"Limit":{"tif":"ioc"}},"reduce_only":false,"cloid":"%s","base_asset_id":"BTC","quote_asset_id":"USDT","maker_fee_bps":5,"taker_fee_bps":10}}}' "$order_id" "$cloid")"

    status="$(
        curl -sS -o /dev/null -w '%{http_code}' \
            -X POST "$LEADER_URL" \
            -H 'content-type: application/json' \
            --data "$body" \
            || true
    )"
    echo "[hotstuff_tcp_demo] sent seq=${seq_no} order_id=${order_id} status=${status}"
}

start_node 0
start_node 1
start_node 2
wait_for_leader_http

echo "[hotstuff_tcp_demo] posting one command every ${INTERVAL_SECONDS}s"
echo "[hotstuff_tcp_demo] tail logs with: tail -f ${LOG_DIR}/node*.log"

seq_no=0
while true; do
    seq_no=$((seq_no + 1))
    send_command "$seq_no"
    sleep "$INTERVAL_SECONDS"
done
