#!/usr/bin/env python3
"""Offline business-definition checker for RustLOB StateMachineV2 use cases."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
DEFAULT_ROOT = ROOT / "lib" / "example" / "use_case"
IMPL_RE = re.compile(r"impl(?:\s*<[^>]*>)?[^\n]*\bStateMachineV2\s+for\s+")
INFRA_RE = re.compile(
    r"\b(sqlx|diesel|reqwest|axum|kafka|redis|repository|database|http|broker)\b",
    re.IGNORECASE,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Score RustLOB StateMachineV2 files as business use cases."
    )
    parser.add_argument("paths", nargs="*", help="Rust files or directories to inspect")
    parser.add_argument("--all", action="store_true", help="inspect all example use cases")
    parser.add_argument("--json", action="store_true", help="emit JSON")
    return parser.parse_args()


def rust_files(paths: list[Path]) -> list[Path]:
    files: set[Path] = set()
    for path in paths:
        if path.is_file() and path.suffix == ".rs":
            files.add(path.resolve())
        elif path.is_dir():
            files.update(candidate.resolve() for candidate in path.rglob("*.rs"))
        else:
            raise ValueError(f"path is not a readable Rust file or directory: {path}")
    return sorted(files)


def inspect(path: Path) -> dict[str, object] | None:
    source = path.read_text(encoding="utf-8")
    if not IMPL_RE.search(source):
        return None

    findings: list[str] = []
    refactors: list[str] = []
    clean = 60
    colors = 40

    required = {
        "check_command": "separate command-only validation in check_command",
        "validate_state_given": "separate loaded-state validation in validate_state_given",
        "compute_state_changed_only": "derive typed after truth in compute_state_changed_only",
        "type StateChanged": "declare a typed StateChanged business truth",
    }
    for token, repair in required.items():
        if token not in source:
            findings.append(f"missing {token}")
            refactors.append(repair)
            clean -= 7

    has_diff = "StateMachineV2Diff" in source
    has_replay = "ReplayableChanges" in source
    if has_diff and not has_replay:
        findings.append("StateMachineV2Diff lacks an explicit ReplayableChanges projection")
        refactors.append("implement ReplayableChanges for the typed StateDiff")
        clean -= 8

    infra_hits = sorted(set(match.group(0).lower() for match in INFRA_RE.finditer(source)))
    if infra_hits:
        findings.append("core use case references outer concerns: " + ", ".join(infra_hits))
        refactors.append("move loading and technical side effects to StateSource, StateSink, or an adapter")
        clean -= min(20, 4 * len(infra_hits))

    command_identity = any(
        token in source for token in ("command_id", "request_id", "client_order_id")
    )
    party_identity = "party_id" in source
    trace_misuse = bool(
        re.search(r"(idempoten|dedup)[^\n]{0,80}trace_id|trace_id[^\n]{0,80}(idempoten|dedup)", source, re.I)
    )
    if not command_identity:
        findings.append("command identity is not explicit in this file")
        refactors.append("carry the stable business command identity in Command when retries matter")
        colors -= 5
    if trace_misuse:
        findings.append("trace_id appears to be used for idempotency or deduplication")
        refactors.append("separate tracing correlation from business command identity")
        colors -= 10

    clean = max(0, clean)
    colors = max(0, colors)
    relative = str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)
    return {
        "path": relative,
        "layer_mapping": {
            "core.use_case": "StateMachineV2 business validation and state transition",
            "core.entity": "reusable entity behavior and invariants",
            "adapter.outbound": "StateSource and StateSink implementations",
            "infra": "database, broker, HTTP, and runtime details",
        },
        "four_color_mapping": {
            "Role": "inspect command party and its business role",
            "Moment-Interval": "one coherent command-to-state-change business action",
            "Party/Place/Thing": "typed StateGiven, StateChanged, and entity pairs",
            "Description": "domain command, error, state, and policy names",
        },
        "identity_semantics": {
            "party_id": "present" if party_identity else "not observed",
            "role()": "review business role naming where applicable",
            "command_id": "present" if command_identity else "not observed",
            "trace_id": "misused" if trace_misuse else "tracing only or not observed",
        },
        "score": {
            "clean_architecture": clean,
            "four_color": colors,
            "total": clean + colors,
        },
        "findings": findings or ["no static business-definition violations found"],
        "minimal_refactor": refactors or ["none"],
    }


def render(result: dict[str, object]) -> str:
    layers = result["layer_mapping"]
    colors = result["four_color_mapping"]
    identity = result["identity_semantics"]
    score = result["score"]
    lines = [
        str(result["path"]),
        "Layer Mapping",
        *(f"- {key}: {value}" for key, value in layers.items()),
        "",
        "四色建模 Mapping",
        *(f"- {key}: {value}" for key, value in colors.items()),
        "",
        "Identity Semantics",
        *(f"- {key}: {value}" for key, value in identity.items()),
        "",
        "Score",
        f"- Clean Architecture: {score['clean_architecture']}/60",
        f"- 四色建模: {score['four_color']}/40",
        f"- Total: {score['total']}/100",
        "",
        "Findings",
        *(f"- {item}" for item in result["findings"]),
        "",
        "Minimal Refactor",
        *(f"- {item}" for item in result["minimal_refactor"]),
    ]
    return "\n".join(lines)


def main() -> int:
    args = parse_args()
    if args.all and args.paths:
        print("--all cannot be combined with explicit paths", file=sys.stderr)
        return 2
    if not args.all and not args.paths:
        print("provide at least one path or use --all", file=sys.stderr)
        return 2

    raw_paths = [DEFAULT_ROOT] if args.all else [Path(item) for item in args.paths]
    paths = [path if path.is_absolute() else ROOT / path for path in raw_paths]
    try:
        files = rust_files(paths)
        results = [result for path in files if (result := inspect(path)) is not None]
    except (OSError, UnicodeError, ValueError) as error:
        print(error, file=sys.stderr)
        return 2

    if not results:
        print("no StateMachineV2 implementation found", file=sys.stderr)
        return 2

    if args.json:
        print(json.dumps(results, ensure_ascii=False, indent=2))
    else:
        print("\n\n".join(render(result) for result in results))

    return 1 if any(result["score"]["total"] < 80 for result in results) else 0


if __name__ == "__main__":
    raise SystemExit(main())
