#!/usr/bin/env python3
"""Offline checker for RustLOB CommandUseCase2 business definition quality."""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent.parent
SCORECARD_PATH = ROOT / ".agents/skills/review-use-case/references/scorecard.md"
DIRECTORY_CANDIDATE_PARENTS = {"workflow", "execute_flow_l4"}
REQUIRED_METHODS = (
    "pre_check_command",
    "validate_against_state",
    "compute_replayable_events",
)

INFRA_PATTERNS: dict[str, str] = {
    "repository": r"\brepositor(?:y|ies)\b",
    "repo": r"\brepo\b",
    "db": r"\bdb\b|\bdatabase\b",
    "mysql": r"\bmysql\b",
    "sqlx": r"\bsqlx\b",
    "mdbx": r"\bmdbx\b",
    "http": r"\bhttp\b",
    "axum": r"\baxum\b",
    "kafka": r"\bkafka\b",
    "nats": r"\bnats\b",
    "sdk": r"\bsdk\b",
    "client": r"\bclient\b",
    "filesystem": r"\bfilesystem\b",
    "tokio_runtime": r"tokio\s*::\s*runtime",
    "runtime": r"\bvmruntime\b|\bruntimeresolver\b|\bvm_resolver\b|\bresolver\b",
}

ORCHESTRATION_PATTERNS: dict[str, str] = {
    "persist": r"\bpersist\b",
    "publish": r"\bpublish\b",
    "replay": r"\breplay\b",
    "load_state": r"\bload_state\b",
    "reply": r"\breply\b|\bresponse\b|\bjson\b",
    "http_reply": r"\bhttpresponse\b|\binto_response\b",
}

TECH_ROLE_TERMS = ("engine", "gateway", "executor", "service", "adapter", "handler", "processor")
GENERIC_NAME_TERMS = ("process", "handle", "data", "result", "engine")
ACTION_TERMS = (
    "place",
    "submit",
    "receive",
    "admit",
    "execute",
    "commit",
    "deposit",
    "withdraw",
    "transfer",
    "create",
    "cancel",
    "open",
    "close",
    "settle",
)
PRECOMPUTED_STATE_TERMS = (
    "accepted",
    "generated",
    "decision",
    "reply",
    "response",
    "result",
    "snapshot",
    "admitted",
    "execution_result",
    "execution_results",
    "committed_block",
    "execution_trace",
    "state_diff",
    "state_changes",
    "block_events",
    "node_state_updates",
    "ingress_decisions",
)
ADAPTERISH_STATE_TERMS = (
    "repo",
    "repository",
    "client",
    "adapter",
    "gateway",
    "handler",
    "processor",
    "resolver",
    "runtime",
    "ctx",
    "context",
    "container",
)


class CheckerError(Exception):
    """Raised when the checker cannot complete due to input or parse errors."""


@dataclass(frozen=True)
class ScorecardWeights:
    clean_architecture: int
    boundary_purity: int
    dependency_direction: int
    responsibility_focus: int
    orchestration_split: int
    four_color: int
    role_clarity: int
    moment_interval_clarity: int
    party_place_thing_modeling: int
    description_quality: int

    @property
    def total(self) -> int:
        return self.clean_architecture + self.four_color


@dataclass
class UseCaseExtraction:
    path: str
    use_case_name: str
    command_type: str
    state_type: str
    error_type: str
    role: str | None
    role_chunk: str
    command_block: str
    state_block: str
    impl_block: str
    method_chunks: dict[str, str]
    command_fields: list[str]
    state_fields: list[str]
    issued_by_party_chunk: str | None
    party_id_chunk: str | None
    file_text: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Check RustLOB CommandUseCase2 business definition quality."
    )
    parser.add_argument("paths", nargs="*", help="Files or directories to check")
    parser.add_argument("--all", action="store_true", help="Scan all candidate use case files")
    parser.add_argument("--json", action="store_true", help="Alias for --format json")
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format (default: text)",
    )
    parser.add_argument(
        "--min-score",
        type=int,
        default=80,
        help="Minimum passing score (default: 80)",
    )
    parser.add_argument(
        "--strict-key-fields",
        dest="strict_key_fields",
        action="store_true",
        default=True,
        help="Require Boundary purity and Role clarity to be non-zero (default: enabled)",
    )
    parser.add_argument(
        "--no-strict-key-fields",
        dest="strict_key_fields",
        action="store_false",
        help="Disable mandatory non-zero Boundary purity and Role clarity checks",
    )
    parser.add_argument(
        "--no-fail",
        action="store_true",
        help="Always exit 0 after reporting unless there is an input or parse error",
    )
    return parser.parse_args()


def parse_scorecard_weights(path: Path) -> ScorecardWeights:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise CheckerError(f"failed to read scorecard: {path}: {exc}") from exc

    def capture(pattern: str) -> int:
        match = re.search(pattern, text, flags=re.MULTILINE)
        if not match:
            raise CheckerError(f"failed to parse scorecard heading: {pattern}")
        return int(match.group(1))

    return ScorecardWeights(
        clean_architecture=capture(r"^## 1\. Clean Architecture: (\d+)"),
        boundary_purity=capture(r"^### 1\.1 Boundary purity: (\d+)"),
        dependency_direction=capture(r"^### 1\.2 Dependency direction: (\d+)"),
        responsibility_focus=capture(r"^### 1\.3 Responsibility focus: (\d+)"),
        orchestration_split=capture(r"^### 1\.4 Orchestration split: (\d+)"),
        four_color=capture(r"^## 2\. .*: (\d+)"),
        role_clarity=capture(r"^### 2\.1 Role clarity: (\d+)"),
        moment_interval_clarity=capture(r"^### 2\.2 Moment-Interval clarity: (\d+)"),
        party_place_thing_modeling=capture(r"^### 2\.3 Party/Place/Thing modeling: (\d+)"),
        description_quality=capture(r"^### 2\.4 Description quality: (\d+)"),
    )


def main() -> int:
    args = parse_args()
    if args.json:
        args.format = "json"

    try:
        weights = parse_scorecard_weights(SCORECARD_PATH)
        candidate_paths, skipped_inputs = collect_targets(args.paths, scan_all=args.all)
        results = [score_file(path, weights, args.min_score, args.strict_key_fields) for path in candidate_paths]
        summary = build_summary(
            args=args,
            results=results,
            skipped_paths=skipped_inputs,
            scorecard_path=SCORECARD_PATH,
        )
        payload = {"summary": summary, "results": results}
        render(payload, args.format)
    except CheckerError as exc:
        render_error(str(exc), output_format=args.format)
        return 2

    if args.no_fail:
        return 0
    return 0 if summary["failed"] == 0 else 1


def collect_targets(input_paths: list[str], *, scan_all: bool) -> tuple[list[Path], list[str]]:
    if scan_all:
        if input_paths:
            raise CheckerError("cannot combine --all with explicit paths")
        return scan_repo(ROOT), []

    if not input_paths:
        raise CheckerError("provide at least one path or use --all")

    discovered: list[Path] = []
    skipped: list[str] = []
    seen: set[Path] = set()

    for raw_path in input_paths:
        path = Path(raw_path)
        if not path.is_absolute():
            path = (ROOT / path).resolve()
        if not path.exists():
            raise CheckerError(f"path does not exist: {raw_path}")
        if path.is_dir():
            for candidate in scan_repo(path):
                if candidate not in seen:
                    discovered.append(candidate)
                    seen.add(candidate)
            continue
        if path.suffix != ".rs":
            skipped.append(str(path.relative_to(ROOT)))
            continue
        if path not in seen:
            discovered.append(path)
            seen.add(path)

    return sorted(discovered), sorted(skipped)


def scan_repo(root: Path) -> list[Path]:
    discovered: list[Path] = []
    for current_root, dirnames, filenames in os.walk(root):
        dirnames[:] = [
            dirname
            for dirname in dirnames
            if not dirname.startswith(".") and dirname not in {"target", "tmp", "__pycache__"}
        ]
        current_path = Path(current_root)
        for filename in filenames:
            if not filename.endswith(".rs"):
                continue
            path = current_path / filename
            if path.parent.name not in DIRECTORY_CANDIDATE_PARENTS:
                continue
            try:
                text = path.read_text(encoding="utf-8")
            except OSError as exc:
                raise CheckerError(f"failed to read {path}: {exc}") from exc
            if "impl CommandUseCase2 for" in text:
                discovered.append(path.resolve())
    return sorted(set(discovered))


def score_file(
    path: Path,
    weights: ScorecardWeights,
    min_score: int,
    strict_key_fields: bool,
) -> dict[str, Any]:
    extraction = extract_use_case(path)
    return build_result(extraction, weights, min_score, strict_key_fields)


def extract_use_case(path: Path) -> UseCaseExtraction:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise CheckerError(f"failed to read {path}: {exc}") from exc

    impl_match = re.search(r"\bimpl\s+CommandUseCase2\s+for\s+([A-Za-z0-9_]+)", text)
    if not impl_match:
        raise CheckerError(f"no `impl CommandUseCase2 for` found in {path}")

    use_case_name = impl_match.group(1)
    impl_start = text.find("{", impl_match.end())
    if impl_start == -1:
        raise CheckerError(f"failed to locate impl body for {use_case_name} in {path}")
    impl_end = find_matching_brace(text, impl_start)
    impl_block = text[impl_start + 1 : impl_end]

    command_type = extract_type_alias(impl_block, "Command", path)
    state_type = extract_type_alias(impl_block, "GivenState", path)
    error_type = extract_type_alias(impl_block, "Error", path)
    method_chunks = extract_method_chunks(impl_block)

    role_chunk = method_chunks.get("role", "")
    role = extract_first_string_literal(role_chunk)

    command_block = extract_named_block(text, command_type)
    state_block = extract_named_block(text, state_type)
    command_fields = extract_fields(command_block)
    state_fields = extract_fields(state_block)

    issued_by_party_chunk = extract_impl_chunk(text, f"IssuedByParty for {command_type}")
    party_id_chunk = (
        method_chunks_from_impl(issued_by_party_chunk).get("party_id") if issued_by_party_chunk else None
    )

    return UseCaseExtraction(
        path=str(path.relative_to(ROOT)),
        use_case_name=use_case_name,
        command_type=command_type,
        state_type=state_type,
        error_type=error_type,
        role=role,
        role_chunk=role_chunk,
        command_block=command_block,
        state_block=state_block,
        impl_block=impl_block,
        method_chunks=method_chunks,
        command_fields=command_fields,
        state_fields=state_fields,
        issued_by_party_chunk=issued_by_party_chunk,
        party_id_chunk=party_id_chunk,
        file_text=text,
    )


def extract_type_alias(impl_block: str, name: str, path: Path) -> str:
    match = re.search(rf"\btype\s+{name}\s*=\s*([^;]+);", impl_block)
    if not match:
        raise CheckerError(f"failed to parse `type {name}` in {path}")
    return match.group(1).strip()


def extract_named_block(text: str, type_name: str) -> str:
    pattern = rf"\b(?:pub\s+)?(?:struct|enum)\s+{re.escape(type_name)}\b"
    match = re.search(pattern, text)
    if not match:
        return ""
    start = text.find("{", match.end())
    if start == -1:
        return ""
    end = find_matching_brace(text, start)
    return text[start + 1 : end]


def extract_impl_chunk(text: str, trait_fragment: str) -> str | None:
    pattern = rf"\bimpl\s+{re.escape(trait_fragment)}\b"
    match = re.search(pattern, text)
    if not match:
        return None
    start = text.find("{", match.end())
    if start == -1:
        return None
    end = find_matching_brace(text, start)
    return text[start + 1 : end]


def extract_method_chunks(impl_block: str) -> dict[str, str]:
    return method_chunks_from_impl(impl_block)


def method_chunks_from_impl(impl_block: str) -> dict[str, str]:
    matches = list(re.finditer(r"(?m)^\s*fn\s+([A-Za-z0-9_]+)\s*\(", impl_block))
    chunks: dict[str, str] = {}
    for index, match in enumerate(matches):
        start = match.start()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(impl_block)
        chunks[match.group(1)] = impl_block[start:end].strip()
    return chunks


def extract_fields(block: str) -> list[str]:
    return sorted(set(re.findall(r"\bpub\s+([A-Za-z0-9_]+)\s*:", block)))


def extract_first_string_literal(text: str) -> str | None:
    match = re.search(r'"([^"\\]*(?:\\.[^"\\]*)*)"', text, flags=re.DOTALL)
    if not match:
        return None
    return bytes(match.group(1), "utf-8").decode("unicode_escape")


def find_matching_brace(text: str, open_index: int) -> int:
    depth = 0
    index = open_index
    length = len(text)
    in_string = False
    in_line_comment = False
    block_comment_depth = 0
    raw_hashes: str | None = None

    while index < length:
        char = text[index]
        next_two = text[index : index + 2]

        if in_line_comment:
            if char == "\n":
                in_line_comment = False
            index += 1
            continue

        if block_comment_depth:
            if next_two == "/*":
                block_comment_depth += 1
                index += 2
                continue
            if next_two == "*/":
                block_comment_depth -= 1
                index += 2
                continue
            index += 1
            continue

        if raw_hashes is not None:
            closing = '"' + raw_hashes
            if text.startswith(closing, index):
                raw_hashes = None
                index += len(closing)
            else:
                index += 1
            continue

        if in_string:
            if char == "\\":
                index += 2
                continue
            if char == '"':
                in_string = False
            index += 1
            continue

        if next_two == "//":
            in_line_comment = True
            index += 2
            continue
        if next_two == "/*":
            block_comment_depth = 1
            index += 2
            continue
        if char == '"':
            in_string = True
            index += 1
            continue
        if char == "r":
            hashes = consume_raw_string_hashes(text, index)
            if hashes is not None:
                raw_hashes = hashes
                index += len(hashes) + 2
                continue
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index
        index += 1

    raise CheckerError("failed to match Rust braces while parsing impl block")


def consume_raw_string_hashes(text: str, index: int) -> str | None:
    if index >= len(text) or text[index] != "r":
        return None
    probe = index + 1
    hashes = ""
    while probe < len(text) and text[probe] == "#":
        hashes += "#"
        probe += 1
    if probe < len(text) and text[probe] == '"':
        return hashes
    return None


def build_result(
    extraction: UseCaseExtraction,
    weights: ScorecardWeights,
    min_score: int,
    strict_key_fields: bool,
) -> dict[str, Any]:
    role_value = extraction.role or "<missing>"
    lowered_file = normalize(extraction.file_text)
    lowered_impl = normalize(extraction.impl_block)
    lowered_role = normalize(role_value)
    lowered_use_case_name = normalize(extraction.use_case_name)
    lowered_command_name = normalize(extraction.command_type)
    lowered_state_name = normalize(extraction.state_type)
    lowered_error_name = normalize(extraction.error_type)

    direct_infra_terms = sorted(match_named_patterns(extraction.file_text, INFRA_PATTERNS))
    compute_chunk = extraction.method_chunks.get("compute_replayable_events", "")
    pre_chunk = extraction.method_chunks.get("pre_check_command", "")
    validate_chunk = extraction.method_chunks.get("validate_against_state", "")
    compute_orchestration_terms = sorted(match_named_patterns(compute_chunk, ORCHESTRATION_PATTERNS))
    has_all_core_methods = all(name in extraction.method_chunks for name in REQUIRED_METHODS)
    pre_uses_state = bool(re.search(r"\bstate\s*\.", pre_chunk))
    validate_uses_state = bool(re.search(r"\bstate\s*\.", validate_chunk))
    compute_uses_cmd = bool(re.search(r"\bcmd\s*\.", compute_chunk))
    compute_uses_state = bool(re.search(r"\bstate\s*\.", compute_chunk))
    compute_uses_self = bool(re.search(r"\bself\s*\.", compute_chunk))
    returns_transport_reply = bool(re.search(r"\bReply\b|\bResponse\b|\bJson\b", compute_chunk))
    role_is_technical = any(term in lowered_role for term in TECH_ROLE_TERMS)
    names_with_generic_terms = sorted(
        {
            term
            for term in GENERIC_NAME_TERMS
            if term in lowered_use_case_name
            or term in lowered_command_name
            or term in lowered_error_name
        }
    )
    command_has_trace_id = any("trace_id" == field or field.endswith("_trace_id") for field in extraction.command_fields)
    command_has_command_id = any("command_id" in field for field in extraction.command_fields)
    has_party_impl = extraction.issued_by_party_chunk is not None
    has_party_id_method = bool(extraction.party_id_chunk)
    has_party_id_signal = bool(
        extraction.party_id_chunk
        and re.search(r"\bSome\s*\(|\bparty_id\b", extraction.party_id_chunk)
        and "None" not in extraction.party_id_chunk
    )
    business_action = looks_like_business_action(extraction.use_case_name, extraction.command_type, extraction.command_fields)
    missing_party_id_semantics = business_action and not has_party_id_signal
    trace_id_as_business_identity = bool(
        re.search(r"stable_entity_id\s*\(\s*&?\s*cmd\s*\.\s*trace_id", compute_chunk)
        or re.search(r"\btrace_id\b.*\bid\b", compute_chunk)
    )
    precomputed_state_signals = sorted(
        {
            term
            for term in PRECOMPUTED_STATE_TERMS
            if term in lowered_state_name or any(term in normalize(field) for field in extraction.state_fields)
        }
    )
    adapterish_state_signals = sorted(
        {
            term
            for term in ADAPTERISH_STATE_TERMS
            if term in lowered_state_name or any(term in normalize(field) for field in extraction.state_fields)
        }
    )
    state_is_precomputed = bool(precomputed_state_signals) and compute_uses_state
    state_is_adapterish = bool(adapterish_state_signals)
    compute_has_event_output = "EntityReplayableEvent" in compute_chunk or "event" in normalize(compute_chunk)

    boundary_purity = weights.boundary_purity
    if direct_infra_terms or returns_transport_reply or compute_orchestration_terms:
        boundary_purity = 0
    elif re.search(r"\btracing\b|\bformat!\b|\bserde\b", extraction.impl_block):
        boundary_purity = 8

    dependency_direction = weights.dependency_direction
    if direct_infra_terms or state_is_adapterish:
        dependency_direction = 0
    elif state_is_precomputed:
        dependency_direction = 8

    responsibility_focus = weights.responsibility_focus
    if not has_all_core_methods or not compute_has_event_output:
        responsibility_focus = 0
    elif pre_uses_state or not validate_uses_state or compute_uses_self or returns_transport_reply:
        responsibility_focus = 8
    if direct_infra_terms and compute_uses_self:
        responsibility_focus = 0

    orchestration_split = weights.orchestration_split
    if compute_orchestration_terms or compute_uses_self and direct_infra_terms:
        orchestration_split = 0
    elif state_is_precomputed or direct_infra_terms:
        orchestration_split = 8

    role_clarity = weights.role_clarity
    if extraction.role is None or role_is_technical:
        role_clarity = 0
    elif "unknown" in lowered_role:
        role_clarity = 5

    moment_interval_clarity = weights.moment_interval_clarity
    if any(term in lowered_use_case_name for term in ("process", "handle")):
        moment_interval_clarity = 0
    elif "and" in split_identifier(extraction.use_case_name) and len(split_identifier(extraction.use_case_name)) > 2:
        moment_interval_clarity = 5

    party_place_thing_modeling = weights.party_place_thing_modeling
    if state_is_adapterish:
        party_place_thing_modeling = 0
    elif state_is_precomputed or any("request" in normalize(field) for field in extraction.state_fields):
        party_place_thing_modeling = 5

    description_quality = weights.description_quality
    if trace_id_as_business_identity or names_with_generic_terms:
        description_quality = 0
    elif command_has_trace_id or role_is_technical or missing_party_id_semantics:
        description_quality = 5

    clean_architecture = (
        boundary_purity + dependency_direction + responsibility_focus + orchestration_split
    )
    four_color = (
        role_clarity + moment_interval_clarity + party_place_thing_modeling + description_quality
    )
    total = clean_architecture + four_color

    downgrade_triggers: list[str] = []
    findings: list[str] = []
    minimal_refactor: list[str] = []

    if direct_infra_terms:
        downgrade_triggers.append("direct infra/adapter terms in use case")
        findings.append(
            "Use case file exposes technical dependencies or vocabulary directly: "
            + ", ".join(direct_infra_terms)
            + "."
        )
        minimal_refactor.append(
            "Move runtime/client/repository concerns behind outbound orchestration and keep the use case on plain domain state."
        )
    if compute_orchestration_terms or returns_transport_reply:
        downgrade_triggers.append("orchestration leakage")
        findings.append(
            "`compute_replayable_events` leaks outer workflow concerns instead of staying on replayable domain events."
        )
        minimal_refactor.append(
            "Keep load/persist/replay/publish/reply mapping outside `compute_replayable_events` and return only domain events."
        )
    if state_is_precomputed:
        downgrade_triggers.append("precomputed state / copied answer")
        findings.append(
            f"`GivenState` looks precomputed ({', '.join(precomputed_state_signals)}), so the use case risks copying prepared answers instead of deriving business decisions."
        )
        minimal_refactor.append(
            "Replace prepared decisions/results in `GivenState` with domain facts and recompute the business decision inside the use case."
        )
    if role_is_technical:
        downgrade_triggers.append("technical role")
        findings.append(f"`role()` returns a technical component name `{role_value}` instead of a business-game role.")
        minimal_refactor.append(
            "Rename `role()` to the real business role carried by the issuing party."
        )
    if missing_party_id_semantics:
        downgrade_triggers.append("missing party_id semantics")
        findings.append(
            "The command looks like a business action but does not clearly carry a business `party_id` through `IssuedByParty`."
        )
        minimal_refactor.append(
            "Carry the issuing business party on the command and implement `IssuedByParty::party_id()` with domain identity."
        )
    if trace_id_as_business_identity:
        downgrade_triggers.append("trace_id used as business identity")
        findings.append(
            "`trace_id` is used as domain identity in event generation, which confuses observability with business idempotency."
        )
        minimal_refactor.append(
            "Use a domain entity key or `command_id` for business identity and keep `trace_id` observability-only."
        )
    if names_with_generic_terms:
        downgrade_triggers.append("business meaning weak")
        findings.append(
            "Key type names hide business meaning behind generic technical terms: "
            + ", ".join(names_with_generic_terms)
            + "."
        )
        minimal_refactor.append(
            "Rename the use case/command/error types to domain-specific business language."
        )
    if not has_all_core_methods:
        downgrade_triggers.append("missing core method split")
        findings.append("The `CommandUseCase2` core method split is incomplete.")
        minimal_refactor.append(
            "Implement the full `pre_check_command` / `validate_against_state` / `compute_replayable_events` separation."
        )
    elif not validate_uses_state:
        downgrade_triggers.append("validate_against_state does not use state")
        findings.append(
            "`validate_against_state` does not visibly validate against `GivenState`, so the separation is weak."
        )
        minimal_refactor.append(
            "Either move state-free checks into `pre_check_command` or validate actual state invariants inside `validate_against_state`."
        )
    if compute_uses_self and direct_infra_terms:
        downgrade_triggers.append("use case computes through technical collaborators")
        findings.append(
            "The use case drives technical collaborators from inside `compute_replayable_events`, mixing business rules with execution machinery."
        )
        minimal_refactor.append(
            "Push technical execution to orchestration and feed the use case a business-shaped snapshot instead."
        )

    findings = dedupe_preserve_order(findings) or ["No major business-definition issues detected by the static heuristic."]
    minimal_refactor = dedupe_preserve_order(minimal_refactor) or [
        "No minimal refactor suggested."
    ]
    downgrade_triggers = dedupe_preserve_order(downgrade_triggers)

    passed = total >= min_score
    if strict_key_fields and (boundary_purity == 0 or role_clarity == 0):
        passed = False

    layer_mapping = {
        "core.workflow": extraction.use_case_name,
        "core.entity": extraction.state_type if extraction.state_type else "<unknown>",
        "adapter.outbound": summarize_adapter_mapping(direct_infra_terms, compute_orchestration_terms),
        "infra": direct_infra_terms or ["no direct infra term detected"],
    }
    four_color_mapping = {
        "role": summarize_role_mapping(role_value, role_clarity),
        "moment_interval": summarize_moment_mapping(extraction.use_case_name, moment_interval_clarity),
        "party_place_thing": summarize_party_mapping(extraction.state_type, extraction.state_fields, party_place_thing_modeling),
        "description": summarize_description_mapping(
            extraction.command_type,
            extraction.error_type,
            names_with_generic_terms,
            description_quality,
        ),
    }
    identity_semantics = {
        "party_id": summarize_party_identity(has_party_impl, has_party_id_method, has_party_id_signal),
        "role()": role_value,
        "command_id": "present on command" if command_has_command_id else "not detected on command",
        "trace_id": summarize_trace_semantics(command_has_trace_id, trace_id_as_business_identity),
    }
    category_scores = {
        "clean_architecture": clean_architecture,
        "boundary_purity": boundary_purity,
        "dependency_direction": dependency_direction,
        "responsibility_focus": responsibility_focus,
        "orchestration_split": orchestration_split,
        "four_color": four_color,
        "role_clarity": role_clarity,
        "moment_interval_clarity": moment_interval_clarity,
        "party_place_thing_modeling": party_place_thing_modeling,
        "description_quality": description_quality,
        "total": total,
    }
    signals = {
        "layer_mapping": layer_mapping,
        "four_color_mapping": four_color_mapping,
        "method_presence": {name: name in extraction.method_chunks for name in REQUIRED_METHODS},
        "method_signals": {
            "pre_check_uses_only_command": not pre_uses_state,
            "validate_uses_state": validate_uses_state,
            "compute_uses_command": compute_uses_cmd,
            "compute_uses_state": compute_uses_state,
            "compute_uses_self": compute_uses_self,
            "compute_has_event_output": compute_has_event_output,
        },
        "identity_signals": {
            "issued_by_party_impl": has_party_impl,
            "party_id_method": has_party_id_method,
            "party_id_semantics": has_party_id_signal,
            "command_has_trace_id": command_has_trace_id,
            "command_has_command_id": command_has_command_id,
            "trace_id_as_business_identity": trace_id_as_business_identity,
        },
        "naming_signals": {
            "generic_terms": names_with_generic_terms,
            "technical_role": role_is_technical,
        },
        "state_signals": {
            "fields": extraction.state_fields,
            "precomputed_terms": precomputed_state_signals,
            "adapterish_terms": adapterish_state_signals,
        },
        "high_risk_terms": direct_infra_terms,
        "rubric_source": str(SCORECARD_PATH.relative_to(ROOT)),
    }

    return {
        "path": extraction.path,
        "use_case_name": extraction.use_case_name,
        "command_type": extraction.command_type,
        "state_type": extraction.state_type,
        "error_type": extraction.error_type,
        "role": role_value,
        "score": total,
        "passed": passed,
        "category_scores": category_scores,
        "identity_semantics": identity_semantics,
        "findings": findings,
        "minimal_refactor": minimal_refactor,
        "signals": signals,
        "downgrade_triggers": downgrade_triggers,
    }


def summarize_adapter_mapping(
    infra_terms: list[str], orchestration_terms: list[str]
) -> str:
    if infra_terms or orchestration_terms:
        joined = ", ".join(infra_terms + orchestration_terms)
        return f"technical boundary leakage detected: {joined}"
    return "no direct outbound orchestration leakage detected"


def summarize_role_mapping(role_value: str, role_clarity: int) -> str:
    if role_clarity == 0:
        return f"{role_value} (technical or missing business role)"
    if role_clarity < 10:
        return f"{role_value} (present but vague)"
    return f"{role_value} (business role looks clear)"


def summarize_moment_mapping(use_case_name: str, score: int) -> str:
    if score == 0:
        return f"{use_case_name} bundles or hides the business moment"
    if score < 10:
        return f"{use_case_name} combines multiple lifecycle verbs in one use case"
    return f"{use_case_name} reads as one coherent business action"


def summarize_party_mapping(state_type: str, state_fields: list[str], score: int) -> str:
    fields = ", ".join(state_fields[:5]) if state_fields else "no state fields extracted"
    if score == 0:
        return f"{state_type} looks adapter- or container-shaped ({fields})"
    if score < 10:
        return f"{state_type} mixes domain objects with precomputed or transport-shaped data ({fields})"
    return f"{state_type} is centered on domain state ({fields})"


def summarize_description_mapping(
    command_type: str,
    error_type: str,
    generic_terms: list[str],
    score: int,
) -> str:
    if score == 0:
        return f"{command_type} / {error_type} hide business meaning with generic or technical naming"
    if score < 10:
        return f"{command_type} / {error_type} are partly business, partly technical"
    if generic_terms:
        return f"{command_type} / {error_type} mostly read as domain language with minor generic terms"
    return f"{command_type} / {error_type} use clear domain language"


def summarize_party_identity(
    has_party_impl: bool, has_party_id_method: bool, has_party_id_signal: bool
) -> str:
    if not has_party_impl:
        return "no `IssuedByParty` implementation detected"
    if not has_party_id_method:
        return "`IssuedByParty` is implemented with default `None` semantics"
    if not has_party_id_signal:
        return "`party_id()` exists but business identity is unclear"
    return "business `party_id` is explicitly carried by the command"


def summarize_trace_semantics(
    command_has_trace_id: bool, trace_id_as_business_identity: bool
) -> str:
    if trace_id_as_business_identity:
        return "present and misused as business identity"
    if command_has_trace_id:
        return "present on command; treat as observability only"
    return "not detected on command"


def looks_like_business_action(use_case_name: str, command_type: str, command_fields: list[str]) -> bool:
    lowered = normalize(use_case_name + " " + command_type)
    if any(term in lowered for term in ACTION_TERMS):
        return True
    non_technical_fields = [
        field
        for field in command_fields
        if field not in {"trace_id", "command_id"}
        and not field.endswith("_trace_id")
        and not field.endswith("_command_id")
    ]
    return bool(non_technical_fields)


def split_identifier(value: str) -> list[str]:
    spaced = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", value.replace("_", " "))
    return [part.lower() for part in spaced.split() if part]


def match_named_patterns(text: str, patterns: dict[str, str]) -> set[str]:
    lowered = normalize(text)
    matches = set()
    for name, pattern in patterns.items():
        if re.search(pattern, lowered):
            matches.add(name)
    return matches


def normalize(value: str) -> str:
    return value.lower()


def dedupe_preserve_order(items: list[str]) -> list[str]:
    seen: set[str] = set()
    ordered: list[str] = []
    for item in items:
        if item in seen:
            continue
        seen.add(item)
        ordered.append(item)
    return ordered


def build_summary(
    *,
    args: argparse.Namespace,
    results: list[dict[str, Any]],
    skipped_paths: list[str],
    scorecard_path: Path,
) -> dict[str, Any]:
    checked = len(results)
    passed = sum(1 for result in results if result["passed"])
    failed = checked - passed
    avg_score = round(sum(result["score"] for result in results) / checked, 2) if checked else None
    return {
        "checked": checked,
        "passed": passed,
        "failed": failed,
        "skipped": len(skipped_paths),
        "skipped_paths": skipped_paths,
        "min_score": args.min_score,
        "strict_key_fields": args.strict_key_fields,
        "format": args.format,
        "all": args.all,
        "no_fail": args.no_fail,
        "average_score": avg_score,
        "scorecard_path": str(scorecard_path.relative_to(ROOT)),
    }


def render(payload: dict[str, Any], output_format: str) -> None:
    if output_format == "json":
        json.dump(payload, sys.stdout, ensure_ascii=False, indent=2)
        sys.stdout.write("\n")
        return

    summary = payload["summary"]
    print("Use Case Business Definition Check")
    print(
        "Summary: "
        f"checked={summary['checked']} passed={summary['passed']} failed={summary['failed']} "
        f"skipped={summary['skipped']} min_score={summary['min_score']} "
        f"strict_key_fields={summary['strict_key_fields']}"
    )
    print(f"Rubric: {summary['scorecard_path']}")
    if summary["skipped_paths"]:
        print("Skipped: " + ", ".join(summary["skipped_paths"]))

    for result in payload["results"]:
        print()
        print(f"== {result['path']} ==")
        print(f"Use Case: {result['use_case_name']}")
        print("Layer Mapping")
        layer_mapping = result["signals"]["layer_mapping"]
        print(f"- core.workflow: {layer_mapping['core.workflow']}")
        print(f"- core.entity: {layer_mapping['core.entity']}")
        print(f"- adapter.outbound: {layer_mapping['adapter.outbound']}")
        infra_value = layer_mapping["infra"]
        if isinstance(infra_value, list):
            infra_text = ", ".join(infra_value)
        else:
            infra_text = str(infra_value)
        print(f"- infra: {infra_text}")

        print("四色建模 Mapping")
        four_color_mapping = result["signals"]["four_color_mapping"]
        print(f"- Role: {four_color_mapping['role']}")
        print(f"- Moment-Interval: {four_color_mapping['moment_interval']}")
        print(f"- Party/Place/Thing: {four_color_mapping['party_place_thing']}")
        print(f"- Description: {four_color_mapping['description']}")

        print("Identity Semantics")
        identity = result["identity_semantics"]
        print(f"- party_id: {identity['party_id']}")
        print(f"- role(): {identity['role()']}")
        print(f"- command_id: {identity['command_id']}")
        print(f"- trace_id: {identity['trace_id']}")

        print("Score")
        scores = result["category_scores"]
        print(f"- Clean Architecture: {scores['clean_architecture']}/60")
        print(f"- 四色建模: {scores['four_color']}/40")
        print(
            f"- Total: {scores['total']}/100 ({'PASS' if result['passed'] else 'FAIL'})"
        )
        if result["downgrade_triggers"]:
            print("- Downgrade Triggers: " + ", ".join(result["downgrade_triggers"]))

        print("Findings")
        for finding in result["findings"]:
            print(f"- {finding}")

        print("Minimal Refactor")
        for suggestion in result["minimal_refactor"]:
            print(f"- {suggestion}")


def render_error(message: str, *, output_format: str) -> None:
    if output_format == "json":
        json.dump({"summary": {"error": message}, "results": []}, sys.stdout, ensure_ascii=False, indent=2)
        sys.stdout.write("\n")
        return
    print(f"error: {message}", file=sys.stderr)


if __name__ == "__main__":
    sys.exit(main())
