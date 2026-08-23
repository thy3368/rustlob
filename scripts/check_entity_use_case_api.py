#!/usr/bin/env python3
"""Static checker for entity/use-case API surface governance."""

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
CONSTRUCTOR_NAMES = {"new", "from_parts", "assemble"}
EXACT_MATERIAL_GETTERS = {"legs", "children", "entries"}
SUFFIX_MATERIAL_PATTERNS = ("_legs", "_children", "_entries")
PREFIX_MATERIAL_PATTERNS = ("all_", "raw_", "iter_")
RAW_COLLECTION_ALLOW_MARKER = "entity-use-case-api: allow-raw-collection"


class CheckerError(Exception):
    """Raised when the checker cannot complete due to input or parse errors."""


@dataclass(frozen=True)
class PublicMethod:
    name: str
    line: int
    signature: str
    return_type: str | None
    doc_comment: str


@dataclass(frozen=True)
class Violation:
    entity: str
    file: str
    line: int
    code: str
    method: str
    message: str

    def to_json(self) -> dict[str, Any]:
        return {
            "entity": self.entity,
            "file": self.file,
            "line": self.line,
            "code": self.code,
            "method": self.method,
            "message": self.message,
        }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Check MinimalBusinessApi entity API surface against architecture policy."
    )
    parser.add_argument("paths", nargs="*", help="Files or directories to check")
    parser.add_argument("--all", action="store_true", help="Scan all Rust files under the repo")
    parser.add_argument("--json", action="store_true", help="Alias for --format json")
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format (default: text)",
    )
    parser.add_argument(
        "--no-fail",
        action="store_true",
        help="Always exit 0 after reporting unless there is an input or parse error",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.json:
        args.format = "json"

    try:
        targets = collect_targets(args.paths, scan_all=args.all)
        violations = check_targets(targets)
        payload = build_payload(targets, violations)
        render(payload, args.format)
    except CheckerError as exc:
        render_error(str(exc), args.format)
        return 2

    if args.no_fail:
        return 0
    return 0 if payload["summary"]["violations"] == 0 else 1


def collect_targets(input_paths: list[str], *, scan_all: bool) -> list[Path]:
    if scan_all:
        if input_paths:
            raise CheckerError("cannot combine --all with explicit paths")
        return scan_repo(ROOT)

    if not input_paths:
        raise CheckerError("provide at least one path or use --all")

    discovered: list[Path] = []
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
            raise CheckerError(f"expected a Rust file or directory, got: {raw_path}")
        if path not in seen:
            discovered.append(path)
            seen.add(path)

    return sorted(discovered)


def scan_repo(root: Path) -> list[Path]:
    discovered: list[Path] = []
    for current_root, dirnames, filenames in os.walk(root):
        dirnames[:] = [
            dirname
            for dirname in dirnames
            if not dirname.startswith(".")
            and dirname not in {"target", "tmp", "__pycache__", "tests", "fixtures"}
        ]
        current_path = Path(current_root)
        for filename in filenames:
            if not filename.endswith(".rs"):
                continue
            path = (current_path / filename).resolve()
            try:
                text = path.read_text(encoding="utf-8")
            except OSError as exc:
                raise CheckerError(f"failed to read {path}: {exc}") from exc
            if "impl Entity for" in text and "MinimalBusinessApi" in text:
                discovered.append(path)
    return sorted(set(discovered))


def check_targets(paths: list[Path]) -> list[Violation]:
    violations: list[Violation] = []
    for path in paths:
        violations.extend(check_file(path))
    return sorted(violations, key=lambda item: (item.file, item.line, item.code, item.method))


def check_file(path: Path) -> list[Violation]:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise CheckerError(f"failed to read {path}: {exc}") from exc

    governed_entities = extract_minimal_business_entities(text, path)
    if not governed_entities:
        return []

    violations: list[Violation] = []
    file_label = str(path.relative_to(ROOT))
    for entity_name in governed_entities:
        methods = extract_inherent_public_methods(text, entity_name)
        for method in methods:
            violation = classify_violation(entity_name, file_label, method)
            if violation is not None:
                violations.append(violation)
    return violations


def extract_minimal_business_entities(text: str, path: Path) -> list[str]:
    names: list[str] = []
    for match in re.finditer(r"\bimpl\s+Entity\s+for\s+([A-Za-z0-9_]+)", text):
        entity_name = match.group(1)
        block = extract_impl_block(text, match.end(), path)
        if "fn use_case_api_surface" not in block:
            continue
        if "MinimalBusinessApi" not in block:
            continue
        names.append(entity_name)
    return names


def extract_inherent_public_methods(text: str, entity_name: str) -> list[PublicMethod]:
    methods: list[PublicMethod] = []
    pattern = re.compile(rf"\bimpl(?:\s*<[^{{>]+>)?\s+{re.escape(entity_name)}\s*\{{")
    for match in pattern.finditer(text):
        block = extract_block_from_open_brace(text, match.end() - 1)
        block_start_line = line_number_for_offset(text, match.start())
        methods.extend(parse_public_methods(block, block_start_line))
    return methods


def parse_public_methods(block: str, block_start_line: int) -> list[PublicMethod]:
    methods: list[PublicMethod] = []
    lines = block.splitlines()
    index = 0
    while index < len(lines):
        current = lines[index].lstrip()
        if not current.startswith("pub fn "):
            index += 1
            continue

        doc_lines: list[str] = []
        probe = index - 1
        while probe >= 0:
            stripped = lines[probe].strip()
            if stripped.startswith("///"):
                doc_lines.insert(0, stripped)
                probe -= 1
                continue
            if not stripped:
                probe -= 1
                continue
            if stripped.startswith("#["):
                probe -= 1
                continue
            break

        signature_lines = [lines[index].strip()]
        end_index = index
        while "{" not in " ".join(signature_lines) and end_index + 1 < len(lines):
            end_index += 1
            signature_lines.append(lines[end_index].strip())

        signature = " ".join(signature_lines)
        name_match = re.search(r"pub fn ([A-Za-z0-9_]+)\s*\(", signature)
        if name_match is None:
            index = end_index + 1
            continue

        name = name_match.group(1)
        return_type = extract_return_type(signature)
        methods.append(
            PublicMethod(
                name=name,
                line=block_start_line + index,
                signature=signature,
                return_type=return_type,
                doc_comment="\n".join(doc_lines),
            )
        )
        index = end_index + 1
    return methods


def extract_return_type(signature: str) -> str | None:
    arrow = signature.find("->")
    if arrow < 0:
        return None

    body = signature[arrow + 2 :]
    brace = body.find("{")
    if brace >= 0:
        body = body[:brace]
    body = body.strip()
    if " where " in body:
        body = body.split(" where ", 1)[0].strip()
    return body or None


def classify_violation(entity: str, file_label: str, method: PublicMethod) -> Violation | None:
    if method.name in CONSTRUCTOR_NAMES:
        return Violation(
            entity=entity,
            file=file_label,
            line=method.line,
            code="public_wide_constructor",
            method=method.name,
            message=(
                f"`{entity}` is `MinimalBusinessApi` but exposes public wide constructor "
                f"`{method.name}`."
            ),
        )

    if method.name in EXACT_MATERIAL_GETTERS:
        return Violation(
            entity=entity,
            file=file_label,
            line=method.line,
            code="material_getter",
            method=method.name,
            message=(
                f"`{entity}` is `MinimalBusinessApi` but exposes raw material getter "
                f"`{method.name}`."
            ),
        )

    if not is_suspicious_collection_name(method.name):
        return None

    if method.return_type is None or not looks_like_collection_return(method.return_type):
        return None

    if has_raw_collection_exemption(method.doc_comment) and is_summary_collection(method.return_type):
        return None

    return Violation(
        entity=entity,
        file=file_label,
        line=method.line,
        code="raw_child_collection",
        method=method.name,
        message=(
            f"`{entity}` is `MinimalBusinessApi` but exposes collection-shaped method "
            f"`{method.name}` with return type `{method.return_type}`."
        ),
    )


def is_suspicious_collection_name(name: str) -> bool:
    if name in EXACT_MATERIAL_GETTERS:
        return True
    if any(name.endswith(suffix) for suffix in SUFFIX_MATERIAL_PATTERNS):
        return True
    return name.startswith(PREFIX_MATERIAL_PATTERNS)


def looks_like_collection_return(return_type: str) -> bool:
    compact = compact_type(return_type)
    if compact.startswith("&["):
        return True
    if compact.startswith("Vec<"):
        return True
    return compact.startswith("implIterator<Item=&")


def has_raw_collection_exemption(doc_comment: str) -> bool:
    return RAW_COLLECTION_ALLOW_MARKER in doc_comment


def is_summary_collection(return_type: str) -> bool:
    compact = compact_type(return_type)
    if compact.startswith("&[") or compact.startswith("Vec<&") or compact.startswith("implIterator<Item=&"):
        return False
    return "Summary" in return_type or "View" in return_type


def compact_type(return_type: str) -> str:
    return re.sub(r"\s+", "", return_type)


def extract_impl_block(text: str, match_end: int, path: Path) -> str:
    open_brace = text.find("{", match_end)
    if open_brace < 0:
        raise CheckerError(f"failed to find impl block start in {path}")
    return extract_block_from_open_brace(text, open_brace)


def extract_block_from_open_brace(text: str, open_brace: int) -> str:
    depth = 0
    for index in range(open_brace, len(text)):
        char = text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return text[open_brace + 1 : index]
    raise CheckerError("failed to find matching closing brace")


def line_number_for_offset(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def build_payload(paths: list[Path], violations: list[Violation]) -> dict[str, Any]:
    return {
        "summary": {
            "files_checked": len(paths),
            "violations": len(violations),
        },
        "violations": [violation.to_json() for violation in violations],
    }


def render(payload: dict[str, Any], output_format: str) -> None:
    if output_format == "json":
        print(json.dumps(payload, ensure_ascii=False, indent=2))
        return

    summary = payload["summary"]
    print(
        f"Checked {summary['files_checked']} file(s), found {summary['violations']} violation(s)."
    )
    for violation in payload["violations"]:
        print(
            f"{violation['file']}:{violation['line']}: "
            f"[{violation['code']}] {violation['message']}"
        )


def render_error(message: str, output_format: str) -> None:
    if output_format == "json":
        print(json.dumps({"error": message}, ensure_ascii=False, indent=2), file=sys.stderr)
        return
    print(f"error: {message}", file=sys.stderr)


if __name__ == "__main__":
    sys.exit(main())
