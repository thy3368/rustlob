#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

CRITERION_DIR="${CRITERION_DIR:-$ROOT_DIR/target/criterion}"
GROUP_FILTER=""
OUTPUT_FORMAT="table"
SAVE_JSON="${SAVE_JSON:-$ROOT_DIR/tmp/use_case_qps_report.json}"

usage() {
    cat <<'EOF'
Usage:
  ./scripts/report_use_case_qps.sh [--criterion-dir DIR] [--group PATTERN] [--json] [--no-save]

Options:
  --criterion-dir DIR   Criterion output directory. Default: target/criterion
  --group PATTERN       Only include benchmark ids containing this pattern
  --json                Print JSON instead of table
  --no-save             Do not save the parsed report to tmp/use_case_qps_report.json
  -h, --help            Show this help

Notes:
  - Reads Criterion benchmark results from */new/benchmark.json and estimates.json
  - command_qps = 1e9 / mean_ns_per_command
  - request_qps = command_qps * requests_per_command
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --criterion-dir)
            CRITERION_DIR="$2"
            shift 2
            ;;
        --group)
            GROUP_FILTER="$2"
            shift 2
            ;;
        --json)
            OUTPUT_FORMAT="json"
            shift
            ;;
        --no-save)
            SAVE_JSON=""
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

if ! command -v jq >/dev/null 2>&1; then
    echo "error: jq is required for scripts/report_use_case_qps.sh" >&2
    exit 2
fi

if [[ ! -d "$CRITERION_DIR" ]]; then
    echo "error: criterion directory not found: $CRITERION_DIR" >&2
    echo "hint: run cargo bench first, e.g. cargo bench -p l1_core --bench use_case_qps -- --noplot" >&2
    exit 2
fi

TMP_JSON="$(mktemp "${TMPDIR:-/tmp}/rustlob-use-case-qps.XXXXXX")"
trap 'rm -f "$TMP_JSON"' EXIT

format_duration_ns() {
    local ns="$1"
    awk -v ns="$ns" '
        BEGIN {
            if (ns < 1000) {
                printf "%.2f ns", ns;
            } else if (ns < 1000000) {
                printf "%.2f us", ns / 1000;
            } else if (ns < 1000000000) {
                printf "%.2f ms", ns / 1000000;
            } else {
                printf "%.2f s", ns / 1000000000;
            }
        }
    '
}

format_rate() {
    local value="$1"
    awk -v value="$value" '
        BEGIN {
            abs = value < 0 ? -value : value;
            if (abs >= 1000000000) {
                printf "%.2fG", value / 1000000000;
            } else if (abs >= 1000000) {
                printf "%.2fM", value / 1000000;
            } else if (abs >= 1000) {
                printf "%.2fK", value / 1000;
            } else {
                printf "%.2f", value;
            }
        }
    '
}

mapfile -t BENCHMARK_FILES < <(find "$CRITERION_DIR" -type f -path '*/new/benchmark.json' | sort)

if ((${#BENCHMARK_FILES[@]} == 0)); then
    echo "error: no Criterion benchmark.json files found under $CRITERION_DIR" >&2
    echo "hint: run cargo bench first, e.g. cargo bench -p l1_core --bench use_case_qps -- --noplot" >&2
    exit 2
fi

{
    for benchmark_file in "${BENCHMARK_FILES[@]}"; do
        estimates_file="${benchmark_file%benchmark.json}estimates.json"
        if [[ ! -f "$estimates_file" ]]; then
            continue
        fi

        jq -n \
            --slurpfile benchmark "$benchmark_file" \
            --slurpfile estimates "$estimates_file" '
            ($benchmark[0]) as $b
            | ($estimates[0]) as $e
            | ($b.throughput.Elements // (($b.value_str | tonumber?) // 1)) as $requests_per_command
            | ($e.mean.point_estimate) as $mean_ns
            | ($e.mean.confidence_interval.lower_bound) as $mean_ns_lower
            | ($e.mean.confidence_interval.upper_bound) as $mean_ns_upper
            | {
                group_id: $b.group_id,
                function_id: ($b.function_id // ""),
                benchmark_id: $b.full_id,
                requests_per_command: $requests_per_command,
                mean_ns_per_command: $mean_ns,
                mean_ns_ci_lower: $mean_ns_lower,
                mean_ns_ci_upper: $mean_ns_upper,
                command_qps: (1000000000 / $mean_ns),
                request_qps: ((1000000000 / $mean_ns) * $requests_per_command),
                command_qps_ci_lower: (1000000000 / $mean_ns_upper),
                command_qps_ci_upper: (1000000000 / $mean_ns_lower),
                request_qps_ci_lower: ((1000000000 / $mean_ns_upper) * $requests_per_command),
                request_qps_ci_upper: ((1000000000 / $mean_ns_lower) * $requests_per_command)
            }
        '
    done
} | jq -s --arg group_filter "$GROUP_FILTER" '
    sort_by(.group_id, .function_id, .requests_per_command)
    | map(
        if $group_filter == "" then .
        else select(.benchmark_id | contains($group_filter))
        end
      )
' >"$TMP_JSON"

if [[ -n "$SAVE_JSON" ]]; then
    mkdir -p "$(dirname "$SAVE_JSON")"
    cp "$TMP_JSON" "$SAVE_JSON"
fi

COUNT="$(jq 'length' "$TMP_JSON")"
if [[ "$COUNT" == "0" ]]; then
    echo "error: no benchmark entries matched the current filter" >&2
    exit 1
fi

if [[ "$OUTPUT_FORMAT" == "json" ]]; then
    cat "$TMP_JSON"
    exit 0
fi

printf 'criterion dir: %s\n' "$CRITERION_DIR"
if [[ -n "$GROUP_FILTER" ]]; then
    printf 'group filter: %s\n' "$GROUP_FILTER"
fi
if [[ -n "$SAVE_JSON" ]]; then
    printf 'json report: %s\n' "$SAVE_JSON"
fi
echo
printf '%-56s %8s %14s %14s %14s\n' \
    "benchmark" "req/cmd" "mean" "cmd_qps" "req_qps"
printf '%-56s %8s %14s %14s %14s\n' \
    "---------" "-------" "----" "-------" "-------"

while IFS=$'\t' read -r benchmark_id requests_per_command mean_ns command_qps request_qps; do
    printf '%-56s %8s %14s %14s %14s\n' \
        "$benchmark_id" \
        "$requests_per_command" \
        "$(format_duration_ns "$mean_ns")" \
        "$(format_rate "$command_qps")" \
        "$(format_rate "$request_qps")"
done < <(
    jq -r '
        .[]
        | [
            .benchmark_id,
            (.requests_per_command | tostring),
            (.mean_ns_per_command | tostring),
            (.command_qps | tostring),
            (.request_qps | tostring)
          ]
        | @tsv
    ' "$TMP_JSON"
)
