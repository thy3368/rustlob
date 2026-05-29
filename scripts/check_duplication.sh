#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

FAILURES=0
WARNINGS=0
TMP_DIR="$ROOT_DIR/tmp"
REPORT_FILE="$TMP_DIR/duplication-report.json"
TOP_N="${TOP_N:-10}"
MIN_LINES="${MIN_LINES:-20}"
MIN_OCCURRENCES="${MIN_OCCURRENCES:-2}"

DEFAULT_EXCLUDES=(
    ".claude/**"
    "target/**"
    "tmp/**"
    "app/xdp_libbpf/libbpf-rs-0.24.8/**"
)

BUSINESS_NOISE_PATHS=(
    "examples/"
    "/examples/"
    "tests/"
    "/tests/"
    "study/"
    "/study/"
)

print_section() {
    printf '\n== %s ==\n' "$1"
}

pass() {
    printf 'PASS: %s\n' "$1"
}

fail() {
    printf 'FAIL: %s\n' "$1"
    FAILURES=$((FAILURES + 1))
}

warn() {
    printf 'WARN: %s\n' "$1"
    WARNINGS=$((WARNINGS + 1))
}

run_command_check() {
    local title="$1"
    shift

    print_section "$title"
    if "$@"; then
        pass "$title"
    else
        fail "$title"
    fi
}

build_cargo_duplicated_cmd() {
    local path="${1:-.}"
    local -n out_ref=$2

    out_ref=(cargo-duplicated "$path" --format json)
    for pattern in "${DEFAULT_EXCLUDES[@]}"; do
        out_ref+=(--exclude "$pattern")
    done
}

print_section "Tooling"
if command -v cargo-duplicated >/dev/null 2>&1; then
    pass "cargo-duplicated is installed"
else
    fail "cargo-duplicated is not installed"
    echo "  install with: cargo install cargo-duplicated"
    echo
    echo "Summary: failures=$FAILURES warnings=$WARNINGS"
    exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
    fail "jq is not installed"
    echo "  install jq to summarize duplication reports"
    echo
    echo "Summary: failures=$FAILURES warnings=$WARNINGS"
    exit 1
fi

mkdir -p "$TMP_DIR"

print_section "Scan"
declare -a DUP_CMD
build_cargo_duplicated_cmd "." DUP_CMD

set +e
"${DUP_CMD[@]}" >"$REPORT_FILE"
DUP_EXIT=$?
set -e

if [[ $DUP_EXIT -eq 0 ]]; then
    pass "cargo-duplicated found no duplicated blocks"
elif [[ $DUP_EXIT -eq 1 ]]; then
    fail "cargo-duplicated found duplicated blocks"
else
    fail "cargo-duplicated failed unexpectedly"
    echo "  report file: $REPORT_FILE"
    echo
    echo "Summary: failures=$FAILURES warnings=$WARNINGS"
    exit "$DUP_EXIT"
fi

print_section "Report"
echo "report file: $REPORT_FILE"
echo "selection: top $TOP_N blocks, min_lines=$MIN_LINES, min_occurrences=$MIN_OCCURRENCES"

jq -r \
    --argjson top_n "$TOP_N" \
    --argjson min_lines "$MIN_LINES" \
    --argjson min_occurrences "$MIN_OCCURRENCES" \
    --argjson business_noise "$(printf '%s\n' "${BUSINESS_NOISE_PATHS[@]}" | jq -R . | jq -s .)" '
    def is_business_noise($occ):
      any($business_noise[]; . as $needle | $occ.file | contains($needle));

    [
      .duplicates[]
      | {
          length,
          occurrence_count: (.occurrences | length),
          occurrences,
          snippet
        }
      | select(.length >= $min_lines)
      | select(.occurrence_count >= $min_occurrences)
      | . + {
          non_noise_occurrences: (.occurrences | map(select(is_business_noise(.) | not))),
          non_noise_count: (.occurrences | map(select(is_business_noise(.) | not)) | length)
        }
      | select(.non_noise_count >= $min_occurrences)
    ]
    | sort_by(-.length, -.non_noise_count)
    | .[:$top_n]
    | if length == 0 then
        "No high-value duplication blocks matched the current thresholds."
      else
        to_entries[]
        | "\(.key + 1). \(.value.length) lines, \(.value.non_noise_count) occurrences\n"
          + (
              .value.non_noise_occurrences
              | map("  - \(.file):\(.start_line)-\(.end_line)")
              | join("\n")
            )
          + "\n"
      end
  ' "$REPORT_FILE"

echo
echo "Summary: failures=$FAILURES warnings=$WARNINGS"

if ((FAILURES > 0)); then
    exit 1
fi
