#!/usr/bin/env bash

set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

FAILURES=0
WARNINGS=0

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

print_section "Tooling"
if command -v cargo-deny >/dev/null 2>&1 || cargo deny --version >/dev/null 2>&1; then
    pass "cargo-deny is installed"
else
    fail "cargo-deny is not installed"
    echo "  install with: cargo install cargo-deny"
    echo
    echo "Summary: failures=$FAILURES warnings=$WARNINGS"
    exit 1
fi

run_command_check \
    "cargo-deny (advisories, licenses, sources, bans)" \
    cargo deny check advisories licenses sources bans --exclude-dev

echo
echo "Summary: failures=$FAILURES warnings=$WARNINGS"

if ((FAILURES > 0)); then
    exit 1
fi
