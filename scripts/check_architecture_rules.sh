#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
CONFIG_FILE="${ARCH_RULES_CONFIG:-$ROOT_DIR/scripts/architecture_rules.json}"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        *)
            echo "error: unknown argument '$1'" >&2
            exit 2
            ;;
    esac
done

if ! command -v jq >/dev/null 2>&1; then
    echo "error: jq is required for scripts/check_architecture_rules.sh" >&2
    exit 2
fi

if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "error: config file not found: $CONFIG_FILE" >&2
    exit 2
fi

FAILURES=0
WARNINGS=0
METADATA_FILE="$(mktemp "${TMPDIR:-/tmp}/rustlob-architecture-rules.XXXXXX")"
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

workspace_crates() {
    jq -r '.packages[].name' "$METADATA_FILE" | sort -u
}

manifest_path_for() {
    local crate="$1"
    jq -r --arg crate "$crate" '
        .packages[]
        | select(.name == $crate)
        | .manifest_path
    ' "$METADATA_FILE"
}

layer_from_config() {
    local crate="$1"
    jq -r --arg crate "$crate" '
        if ((.crate_layers.core // []) | index($crate)) != null then "core"
        elif ((.crate_layers.adapter_inbound // []) | index($crate)) != null then "adapter:inbound"
        elif ((.crate_layers.adapter_outbound // []) | index($crate)) != null then "adapter:outbound"
        elif ((.crate_layers.infra // []) | index($crate)) != null then "infra"
        else empty
        end
    ' "$CONFIG_FILE"
}

layer_for() {
    local crate="$1"
    local configured_layer
    configured_layer="$(layer_from_config "$crate")"
    if [[ -n "$configured_layer" ]]; then
        echo "$configured_layer"
        return
    fi

    local manifest_path
    manifest_path="$(manifest_path_for "$crate")"

    case "$manifest_path" in
        */lib/core/l1_adapter/*) echo "adapter:outbound" ;;
        */lib/core/l1_e2e/*) echo "infra" ;;
        */lib/core/l1/*|*/lib/core/exchange/prep/*) echo "core" ;;
        */lib/core/*) echo "core" ;;
        */inbound_adapter/*) echo "adapter:inbound" ;;
        */lib/common/mdbx/*|*/app/*|*/study/*) echo "infra" ;;
        */lib/common/db_repo/*|*/operating/*|*/lib/common/lob_repo/*|*/lib/common/rust_queue/*|*/lib/common/sbe/*)
            echo "adapter:outbound"
            ;;
        */lib/common/*)
            echo "shared"
            ;;
        *)
            echo "unknown"
            ;;
    esac
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

matches_exact_or_pattern() {
    local value="$1"
    local exact="$2"
    local pattern="$3"

    if [[ -n "$exact" ]]; then
        [[ "$value" == "$exact" ]]
        return
    fi

    if [[ -n "$pattern" ]]; then
        [[ "$value" == $pattern ]]
        return
    fi

    return 1
}

rule_target_label() {
    local exact="$1"
    local pattern="$2"

    if [[ -n "$exact" ]]; then
        printf '%s' "$exact"
    else
        printf '%s' "$pattern"
    fi
}

has_direct_dep() {
    local crate="$1"
    local dep="$2"
    direct_normal_deps "$crate" | rg -x --quiet "$dep"
}

allowed_dep_reason() {
    local crate="$1"
    local dep="$2"

    while IFS=$'\x1f' read -r from from_pattern to to_pattern reason; do
        if matches_exact_or_pattern "$crate" "$from" "$from_pattern" &&
            matches_exact_or_pattern "$dep" "$to" "$to_pattern"; then
            printf '%s\n' "$reason"
            return 0
        fi
    done < <(
        jq -r '
            (.allow_direct_deps // [])[]
            | [(.from // ""), (.from_pattern // ""), (.to // ""), (.to_pattern // ""), .reason]
            | join("\u001f")
        ' "$CONFIG_FILE"
    )

    return 1
}

assert_no_direct_dep() {
    local crate="$1"
    local dep="$2"
    local reason="$3"

    if allow_reason="$(allowed_dep_reason "$crate" "$dep")"; then
        pass "$crate -> $dep allowed by exception ($allow_reason)"
    else
        fail "$crate must not directly depend on $dep ($reason)"
    fi
}

assert_has_direct_dep() {
    local from="$1"
    local from_pattern="$2"
    local to="$3"
    local to_pattern="$4"
    local reason="$5"
    local from_label
    local to_label
    local matched_crates=0

    from_label="$(rule_target_label "$from" "$from_pattern")"
    to_label="$(rule_target_label "$to" "$to_pattern")"

    while IFS= read -r crate; do
        local found=0

        if ! matches_exact_or_pattern "$crate" "$from" "$from_pattern"; then
            continue
        fi

        matched_crates=1
        while IFS= read -r dep; do
            if matches_exact_or_pattern "$dep" "$to" "$to_pattern"; then
                found=1
                break
            fi
        done < <(direct_normal_deps "$crate")

        if ((found)); then
            pass "$crate directly depends on $to_label"
        else
            fail "$crate should directly depend on $to_label ($reason)"
        fi
    done < <(workspace_crates)

    if ((matched_crates == 0)); then
        warn "require_direct_deps rule matched no crates: $from_label -> $to_label"
    fi
}

check_forbid_direct_dep_rule() {
    local crate="$1"
    local dep="$2"

    while IFS=$'\x1f' read -r from from_pattern to to_pattern reason; do
        if matches_exact_or_pattern "$crate" "$from" "$from_pattern" &&
            matches_exact_or_pattern "$dep" "$to" "$to_pattern"; then
            assert_no_direct_dep "$crate" "$dep" "$reason"
        fi
    done < <(
        jq -r '
            (.forbid_direct_deps // [])[]
            | [(.from // ""), (.from_pattern // ""), (.to // ""), (.to_pattern // ""), .reason]
            | join("\u001f")
        ' "$CONFIG_FILE"
    )
}

assert_layer_direction() {
    local crate="$1"
    local dep="$2"
    local crate_layer="$3"
    local dep_layer="$4"

    while IFS=$'\t' read -r from_layer to_layer reason; do
        if [[ "$crate_layer" == "$from_layer" && "$dep_layer" == "$to_layer" ]]; then
            if allow_reason="$(allowed_dep_reason "$crate" "$dep")"; then
                pass "$crate -> $dep allowed by exception ($allow_reason)"
            else
                fail "$crate ($crate_layer) must not directly depend on $dep ($dep_layer) ($reason)"
            fi
        fi
    done < <(
        jq -r '
            (.forbid_layer_deps // [])[]
            | [.from_layer, .to_layer, .reason]
            | @tsv
        ' "$CONFIG_FILE"
    )
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

print_section "Config"
printf 'config file: %s\n' "$CONFIG_FILE"

print_section "Layer Snapshot"
while IFS= read -r crate; do
    printf '%s\t%s\n' "$crate" "$(layer_for "$crate")"
done < <(jq -r '(.focus_crates // [])[]' "$CONFIG_FILE")

print_section "Explicit Rules"
while IFS=$'\x1f' read -r from from_pattern to to_pattern reason; do
    assert_has_direct_dep "$from" "$from_pattern" "$to" "$to_pattern" "$reason"
done < <(
    jq -r '
        (.require_direct_deps // [])[]
        | [(.from // ""), (.from_pattern // ""), (.to // ""), (.to_pattern // ""), .reason]
        | join("\u001f")
    ' "$CONFIG_FILE"
)

while IFS=$'\t' read -r crate dep; do
    check_forbid_direct_dep_rule "$crate" "$dep"
done < <(
    jq -r '
        .packages[]
        | .name as $crate
        | .dependencies[]
        | select(.kind == null)
        | [$crate, .name]
        | @tsv
    ' "$METADATA_FILE"
)

print_section "Generic Layer Direction"
while IFS=$'\t' read -r crate dep; do
    crate_layer="$(layer_for "$crate")"
    dep_layer="$(layer_for "$dep")"
    assert_layer_direction "$crate" "$dep" "$crate_layer" "$dep_layer"
done < <(
    jq -r '
        .packages[]
        | .name as $crate
        | .dependencies[]
        | select(.kind == null)
        | [$crate, .name]
        | @tsv
    ' "$METADATA_FILE"
)

print_section "High Fan-out"
HIGH_FANOUT_THRESHOLD="$(jq -r '.high_fanout_threshold // 10' "$CONFIG_FILE")"
jq -r '
    .packages[]
    | {
        name,
        dep_count: ([.dependencies[] | select(.kind == null)] | length)
      }
    | select(.dep_count >= ($threshold | tonumber))
    | "\(.name)\t\(.dep_count)"
' --arg threshold "$HIGH_FANOUT_THRESHOLD" "$METADATA_FILE" | sort -k2,2nr | sed -n '1,15p'

print_section "Direct Dependency Snapshot"
while IFS= read -r crate; do
    printf '%s: %s\n' "$crate" "$(direct_normal_deps "$crate" | tr '\n' ' ')"
done < <(jq -r '(.focus_crates // [])[]' "$CONFIG_FILE")

echo
echo "Summary: failures=$FAILURES warnings=$WARNINGS"

if ((FAILURES > 0)); then
    exit 1
fi
