#!/usr/bin/env bash

set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if ! command -v jq >/dev/null 2>&1; then
    echo "error: jq is required for scripts/check_architecture.sh" >&2
    exit 2
fi

FAILURES=0
WARNINGS=0
METADATA_FILE="$(mktemp /tmp/rustlob-architecture-metadata.XXXXXX.json)"
trap 'rm -f "$METADATA_FILE"' EXIT

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

direct_normal_deps() {
    local crate="$1"
    jq -r --arg crate "$crate" '
        .packages[]
        | select(.name == $crate)
        | .dependencies[]
        | select(.kind == null)
        | .name
    ' "$METADATA_FILE" | sort -u
}

has_direct_dep() {
    local crate="$1"
    local dep="$2"
    direct_normal_deps "$crate" | rg -x --quiet "$dep"
}

assert_no_direct_dep() {
    local crate="$1"
    local dep="$2"
    local reason="$3"

    if has_direct_dep "$crate" "$dep"; then
        fail "$crate must not directly depend on $dep ($reason)"
    else
        pass "$crate does not directly depend on $dep"
    fi
}

assert_has_direct_dep() {
    local crate="$1"
    local dep="$2"
    local reason="$3"

    if has_direct_dep "$crate" "$dep"; then
        pass "$crate directly depends on $dep"
    else
        fail "$crate should directly depend on $dep ($reason)"
    fi
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

print_section "Metadata"
if cargo metadata --format-version 1 --no-deps >"$METADATA_FILE"; then
    pass "cargo metadata loaded"
else
    fail "cargo metadata failed"
    echo
    echo "Summary: failures=$FAILURES warnings=$WARNINGS"
    exit 1
fi

print_section "Layer Boundary Rules"
assert_no_direct_dep "l1_core" "l1_adapter" "core must not depend on adapter"
assert_no_direct_dep "l1_core" "revm" "core must not depend on EVM infra"
assert_no_direct_dep "l1_core" "mdbx" "core must not depend on storage infra"
assert_no_direct_dep "l1_core" "axum" "core must not depend on HTTP framework"
assert_no_direct_dep "l1_core" "mysql" "core must not depend on database driver"
assert_no_direct_dep "l1_core" "tokio" "core should stay runtime-agnostic"

assert_has_direct_dep "l1_adapter" "l1_core" "adapter should implement core ports"
assert_no_direct_dep "l1_adapter" "axum" "adapter should not depend on inbound web framework"
assert_no_direct_dep "l1_adapter" "mysql" "adapter should not mix storage technologies arbitrarily"

assert_no_direct_dep "axum_server" "db_repo" "inbound adapter should call use cases, not repositories"
assert_no_direct_dep "axum_server" "lob_repo" "inbound adapter should not own domain persistence"
assert_no_direct_dep "axum_server" "mysql" "inbound adapter should not talk to SQL driver directly"
assert_no_direct_dep "axum_server" "mdbx" "inbound adapter should not depend on storage engine directly"

print_section "Direct Dependency Snapshot"
printf 'l1_core: %s\n' "$(direct_normal_deps "l1_core" | tr '\n' ' ')"
printf 'l1_adapter: %s\n' "$(direct_normal_deps "l1_adapter" | tr '\n' ' ')"
printf 'axum_server: %s\n' "$(direct_normal_deps "axum_server" | tr '\n' ' ')"

run_command_check \
    "Compile-fail boundary tests (l1_core)" \
    cargo test -p l1_core --test architecture

run_command_check \
    "Focused clippy (l1_core, l1_adapter, dex, axum_server)" \
    cargo clippy -p l1_core -p l1_adapter -p dex -p axum_server --all-targets -- -D warnings

print_section "Workspace Hygiene"
mapfile -t PROFILE_OFFENDERS < <(
    rg -l '^\[profile\.release\]' --glob '**/Cargo.toml' . \
        | sort \
        | grep -v '^./Cargo.toml$' || true
)

if ((${#PROFILE_OFFENDERS[@]} == 0)); then
    pass "release profile is centralized at the workspace root"
else
    warn "member manifests define [profile.release]; move them to workspace root"
    printf '  %s\n' "${PROFILE_OFFENDERS[@]}"
fi

echo
echo "Summary: failures=$FAILURES warnings=$WARNINGS"

if ((FAILURES > 0)); then
    exit 1
fi
