#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PACKAGE="${PACKAGE:-l1_core}"
BENCH_NAME="${BENCH_NAME:-use_case_qps}"
CRITERION_DIR="${CRITERION_DIR:-$ROOT_DIR/target/criterion}"
GROUP_FILTER=""
OUTPUT_JSON=0

usage() {
    cat <<'EOF'
Usage:
  ./scripts/run_use_case_qps.sh [--package PKG] [--bench NAME] [--group PATTERN] [--json]

Options:
  --package PKG    Cargo package name. Default: l1_core
  --bench NAME     Cargo bench target name. Default: use_case_qps
  --group PATTERN  Forwarded to report_use_case_qps.sh
  --json           Print JSON report instead of table
  -h, --help       Show this help

Examples:
  ./scripts/run_use_case_qps.sh
  ./scripts/run_use_case_qps.sh --group executor.execute
  ./scripts/run_use_case_qps.sh --package my_core --bench my_use_case_qps
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --package)
            PACKAGE="$2"
            shift 2
            ;;
        --bench)
            BENCH_NAME="$2"
            shift 2
            ;;
        --group)
            GROUP_FILTER="$2"
            shift 2
            ;;
        --json)
            OUTPUT_JSON=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown argument '$1'" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [[ ! -x "$ROOT_DIR/scripts/report_use_case_qps.sh" ]]; then
    echo "error: scripts/report_use_case_qps.sh is missing or not executable" >&2
    exit 2
fi

echo "== Benchmark =="
echo "package: $PACKAGE"
echo "bench: $BENCH_NAME"
echo "criterion dir: $CRITERION_DIR"
echo

cargo bench -p "$PACKAGE" --bench "$BENCH_NAME" -- --noplot

echo
echo "== QPS Report =="

REPORT_ARGS=(--criterion-dir "$CRITERION_DIR")
if [[ -n "$GROUP_FILTER" ]]; then
    REPORT_ARGS+=(--group "$GROUP_FILTER")
fi
if ((OUTPUT_JSON)); then
    REPORT_ARGS+=(--json)
fi

"$ROOT_DIR/scripts/report_use_case_qps.sh" "${REPORT_ARGS[@]}"
