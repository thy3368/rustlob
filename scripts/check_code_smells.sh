#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TMP_DIR="$ROOT_DIR/tmp"
REPORT_FILE="$TMP_DIR/code-smells-report.txt"
TOP_N="${TOP_N:-10}"
LONG_FILE_LINES="${LONG_FILE_LINES:-400}"
TODO_THRESHOLD="${TODO_THRESHOLD:-20}"
HIGH_FANOUT_THRESHOLD="${HIGH_FANOUT_THRESHOLD:-10}"

EXCLUDE_GLOBS=(
    ".claude/**"
    "target/**"
    "tmp/**"
    "study/**"
    "app/xdp_libbpf/libbpf-rs-0.24.8/**"
)

REPORT_PIPE_FILE=""
REPORT_TEE_PID=""

print_section() {
    printf '\n== %s ==\n' "$1"
}

pass() {
    printf 'PASS: %s\n' "$1"
}

warn() {
    printf 'WARN: %s\n' "$1"
}

fail() {
    printf 'FAIL: %s\n' "$1"
}

tool_status() {
    local tool="$1"
    if command -v "$tool" >/dev/null 2>&1; then
        pass "$tool is installed"
        return 0
    fi

    warn "$tool is not installed"
    return 1
}

rust_files() {
    rg --files -g '*.rs' | rg -v '^(\.claude/|target/|tmp/|study/|app/xdp_libbpf/libbpf-rs-0\.24\.8/|app/xdp_libbpf/src/bpf/.*\.skel\.rs$)'
}

non_test_rust_files() {
    rust_files | rg -v '(^|/)(tests?|benches|examples)/|_test\.rs$|/test_'
}

cleanup_report() {
    if [[ -n "$REPORT_PIPE_FILE" ]]; then
        rm -f "$REPORT_PIPE_FILE"
    fi
    if [[ -n "$REPORT_TEE_PID" ]]; then
        kill "$REPORT_TEE_PID" 2>/dev/null || true
    fi
}

write_report() {
    mkdir -p "$TMP_DIR"
    : >"$REPORT_FILE"
    REPORT_PIPE_FILE="$(mktemp -u "${TMPDIR:-/tmp}/rustlob-code-smells.XXXXXX")"
    mkfifo "$REPORT_PIPE_FILE"
    tee "$REPORT_FILE" <"$REPORT_PIPE_FILE" &
    REPORT_TEE_PID=$!
    trap cleanup_report EXIT
    exec >"$REPORT_PIPE_FILE" 2>&1
}

report_long_files() {
    print_section "Long Rust Files"

    mapfile -t lines < <(
        rust_files | while IFS= read -r file; do
            wc -l <"$file" | tr -d ' '
            printf '\t%s\n' "$file"
        done | paste - - | sort -rn | sed -n "1,${TOP_N}p"
    )

    if ((${#lines[@]} == 0)); then
        warn "no Rust files found"
        return
    fi

    printf 'threshold: %s lines\n' "$LONG_FILE_LINES"
    for entry in "${lines[@]}"; do
        local count file
        count="${entry%%$'\t'*}"
        file="${entry#*$'\t'}"
        if ((count >= LONG_FILE_LINES)); then
            fail "$file ($count lines)"
        else
            pass "$file ($count lines)"
        fi
    done
}

report_todo_density() {
    print_section "TODO FIXME HACK"

    local total
    total="$(rg -n 'TODO|FIXME|HACK|XXX' \
        --glob '!*.md' \
        --glob '!.claude/**' \
        --glob '!target/**' \
        --glob '!tmp/**' \
        --glob '!study/**' \
        --glob '!myskill/**' \
        --glob '!scripts/**' \
        --glob '!app/xdp_libbpf/libbpf-rs-0.24.8/**' \
        --glob '!app/xdp_libbpf/src/bpf/*.skel.rs' \
        --glob '!scripts/check_code_smells.sh' \
        . | wc -l | tr -d ' ')"

    printf 'total markers: %s\n' "$total"
    if ((total >= TODO_THRESHOLD)); then
        fail "technical-debt markers exceed threshold ($TODO_THRESHOLD)"
    else
        pass "technical-debt markers within threshold"
    fi

    rg -n 'TODO|FIXME|HACK|XXX' \
        --glob '!*.md' \
        --glob '!.claude/**' \
        --glob '!target/**' \
        --glob '!tmp/**' \
        --glob '!study/**' \
        --glob '!myskill/**' \
        --glob '!scripts/**' \
        --glob '!app/xdp_libbpf/libbpf-rs-0.24.8/**' \
        --glob '!app/xdp_libbpf/src/bpf/*.skel.rs' \
        --glob '!scripts/check_code_smells.sh' \
        . | sed -n "1,${TOP_N}p" || true
}

report_unwrap_expect() {
    print_section "unwrap expect panic"

    local total
    total="$(
        non_test_rust_files | xargs rg -n '\.(unwrap|expect)\(|panic!\(' 2>/dev/null | wc -l | tr -d ' '
    )"
    printf 'production matches: %s\n' "$total"
    if ((total > 0)); then
        fail "found panic-prone calls in non-test Rust files"
        non_test_rust_files | xargs rg -n '\.(unwrap|expect)\(|panic!\(' 2>/dev/null | sed -n "1,${TOP_N}p" || true
    else
        pass "no unwrap/expect/panic found in non-test Rust files"
    fi
}

report_high_fanout() {
    print_section "High Fan-out Crates"

    local metadata_file
    metadata_file="$(mktemp "${TMPDIR:-/tmp}/rustlob-smells-metadata.XXXXXX")"

    cargo metadata --format-version 1 --no-deps >"$metadata_file"

    jq -r --arg threshold "$HIGH_FANOUT_THRESHOLD" '
        .packages[]
        | {
            name,
            dep_count: ([.dependencies[] | select(.kind == null)] | length)
          }
        | select(.dep_count >= ($threshold | tonumber))
        | "\(.dep_count)\t\(.name)"
    ' "$metadata_file" | sort -rn | sed -n "1,${TOP_N}p" | while IFS=$'\t' read -r dep_count name; do
        fail "$name has high fan-out ($dep_count direct deps)"
    done

    rm -f "$metadata_file"
}

report_duplicate_code() {
    print_section "Duplicate Code"

    if ! command -v cargo-duplicated >/dev/null 2>&1; then
        warn "cargo-duplicated missing; install with: cargo install cargo-duplicated"
        return
    fi

    local duplication_output
    duplication_output="$(./scripts/check_duplication.sh 2>&1 || true)"
    printf '%s\n' "$duplication_output" | sed -n '/== Report ==/,$p' | sed -n "1,$((TOP_N * 6))p"
}

report_architecture_smells() {
    print_section "Architecture Boundary Smells"
    ./scripts/check_architecture_rules.sh || true
}

report_optional_complexity_tools() {
    print_section "Optional Tooling"

    if command -v rust-code-analysis >/dev/null 2>&1; then
        pass "rust-code-analysis is installed"
        echo "suggestion: rust-code-analysis metrics ./lib ./app ./operating --output-format json"
    else
        warn "rust-code-analysis is not installed"
        echo "  install with: cargo install rust-code-analysis"
    fi

    if command -v cargo-modules >/dev/null 2>&1; then
        pass "cargo-modules is installed"
        echo "suggestion: cargo modules dependencies --no-types"
    else
        warn "cargo-modules is not installed"
        echo "  install with: cargo install cargo-modules"
    fi

    if command -v semgrep >/dev/null 2>&1; then
        pass "semgrep is installed"
        echo "suggestion: semgrep scan --config auto"
    else
        warn "semgrep is not installed"
        echo "  optional: brew install semgrep"
    fi
}

write_report

print_section "Tooling"
tool_status jq || exit 2
tool_status cargo
tool_status cargo-clippy
tool_status cargo-duplicated || true
tool_status rust-code-analysis || true
tool_status cargo-modules || true
tool_status semgrep || true

report_long_files
report_todo_density
report_unwrap_expect
report_high_fanout
report_duplicate_code
report_architecture_smells
report_optional_complexity_tools

echo
echo "report file: $REPORT_FILE"
