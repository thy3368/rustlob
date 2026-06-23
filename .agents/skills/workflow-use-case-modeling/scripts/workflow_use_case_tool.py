#!/usr/bin/env python3
"""Generate and review RustLOB workflow/use-case design skeletons."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[4]
ROOT_CHECKER = ROOT / "scripts" / "check_use_case_business_definition.py"
GENERIC_MODULE_TERMS = {"workflow", "execute_flow_l4", "handler", "service", "manager", "engine"}
TECH_ROLE_TERMS = ("engine", "gateway", "executor", "service", "adapter", "handler", "processor")
QUERY_TERMS = ("get", "list", "query", "view", "fetch", "read")
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
    "register",
    "approve",
    "reject",
    "allocate",
    "freeze",
    "release",
)


@dataclass
class ChoiceQuestion:
    id: str
    question: str
    options: list[str]
    recommended: str
    impact: str


@dataclass
class UseCaseSpec:
    title: str
    use_case_name: str
    file_name: str
    kind: str
    role: str
    command_name: str
    state_name: str
    error_name: str
    output_name: str
    reply_name: str
    reply_mapper_name: str


@dataclass
class WorkflowSpec:
    workflow_name: str
    module_name: str
    domain_module: str
    role: str
    use_cases: list[UseCaseSpec] = field(default_factory=list)


@dataclass
class GeneratePayload:
    status: str
    assumptions: list[str]
    questions: list[ChoiceQuestion]
    workflow: WorkflowSpec | None
    tree: list[str]
    files: dict[str, str]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Generate or review workflow/use-case code skeletons.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    generate = subparsers.add_parser("generate", help="Generate module and use case skeletons")
    generate.add_argument("input", nargs="?", help="Input file path")
    generate.add_argument("--text", help="Inline business text")
    generate.add_argument("--domain", help="Owning domain module, for example l1 or exchange")
    generate.add_argument(
        "--format", choices=("text", "json"), default="text", help="Output format (default: text)"
    )
    generate.add_argument("--json", action="store_true", help="Alias for --format json")
    generate.add_argument(
        "--force-assumptions",
        action="store_true",
        help="Generate code using recommended defaults even when questions remain",
    )

    review = subparsers.add_parser("review", help="Review existing Rust use cases")
    review.add_argument("paths", nargs="*", help="Files or directories to review")
    review.add_argument("--all", action="store_true", help="Review all candidate use case files")
    review.add_argument("--min-score", type=int, default=80, help="Passing score threshold")
    review.add_argument("--format", choices=("text", "json"), default="text")
    review.add_argument("--json", action="store_true", help="Alias for --format json")
    review.add_argument("--no-fail", action="store_true", help="Always exit 0 after reporting")

    return parser


def main() -> int:
    args = build_parser().parse_args()
    if getattr(args, "json", False):
        args.format = "json"

    if args.command == "generate":
        text = load_input_text(args.input, args.text)
        payload = build_generate_payload(text, domain=args.domain, force_assumptions=args.force_assumptions)
        render_generate(payload, args.format)
        if payload.status == "needs_user_choice" and not args.force_assumptions:
            return 2
        return 0

    payload = build_review_payload(args.paths, scan_all=args.all, min_score=args.min_score)
    render_review(payload, args.format)
    if args.no_fail:
        return 0
    return 0 if payload["summary"]["failed"] == 0 else 1


def load_input_text(path_arg: str | None, text_arg: str | None) -> str:
    if text_arg:
        return text_arg.strip()
    if path_arg:
        return Path(path_arg).read_text(encoding="utf-8").strip()
    return sys.stdin.read().strip()


def build_generate_payload(text: str, *, domain: str | None, force_assumptions: bool) -> GeneratePayload:
    explicit_domain = match_value(text, ("domain", "context", "bounded context"))
    explicit_workflow = match_value(text, ("workflow", "process"))
    explicit_role = match_value(text, ("role", "actor", "performer"))
    actions = extract_actions(text)

    if not actions:
        actions = ["Handle business workflow"]

    workflow_title = explicit_workflow or f"{actions[0]} Workflow"
    workflow_name = to_pascal_case(workflow_title, suffix="Workflow")
    module_name = sanitize_module_name(workflow_title)
    role = titleize_role(explicit_role or guess_role(actions))
    domain_module = sanitize_module_name(domain or explicit_domain or "domain_module")

    questions: list[ChoiceQuestion] = []
    assumptions: list[str] = []

    if not (domain or explicit_domain):
        questions.append(
            ChoiceQuestion(
                id="domain_module",
                question="Which existing domain module should own this workflow?",
                options=["l1", "exchange", "wallet"],
                recommended="l1",
                impact="This decides the top-level Rust path that owns the generated module.",
            )
        )
        assumptions.append("No explicit domain module was provided; default placeholder `domain_module` was used.")

    if not explicit_role:
        questions.append(
            ChoiceQuestion(
                id="role",
                question="Which business role should own role() for these use cases?",
                options=[role, "Trader", "Operator"],
                recommended=role,
                impact="This decides the business actor encoded in role() and command ownership semantics.",
            )
        )
        assumptions.append(f"No explicit business role was provided; inferred `{role}` from the action text.")

    use_cases = build_use_cases(actions, role)
    workflow = WorkflowSpec(
        workflow_name=workflow_name,
        module_name=module_name,
        domain_module=domain_module,
        role=role,
        use_cases=use_cases,
    )

    if questions and not force_assumptions:
        return GeneratePayload(
            status="needs_user_choice",
            assumptions=assumptions,
            questions=questions,
            workflow=workflow,
            tree=[],
            files={},
        )

    tree, files = render_workflow_code(workflow)
    if questions and force_assumptions:
        assumptions.append("`--force-assumptions` was used, so recommended defaults were applied.")

    return GeneratePayload(
        status="ready_to_generate",
        assumptions=assumptions,
        questions=questions,
        workflow=workflow,
        tree=tree,
        files=files,
    )


def build_use_cases(actions: list[str], role: str) -> list[UseCaseSpec]:
    use_cases: list[UseCaseSpec] = []
    seen_names: set[str] = set()

    for action in actions:
        stem = to_pascal_case(action, suffix="UseCase")
        while stem in seen_names:
            stem = f"{stem}Next"
        seen_names.add(stem)
        base = stem.removesuffix("UseCase")
        file_name = to_snake_case(base)
        kind = "query" if begins_with_query_term(action) else "command"
        use_cases.append(
            UseCaseSpec(
                title=action,
                use_case_name=stem,
                file_name=file_name,
                kind=kind,
                role=role,
                command_name=f"{base}{'Query' if kind == 'query' else 'Cmd'}",
                state_name=f"{base}StateSnapshot",
                error_name=f"{base}Error",
                output_name=f"{base}Output",
                reply_name=f"{base}Reply",
                reply_mapper_name=f"{base}ReplyMapper",
            )
        )
    return use_cases


def begins_with_query_term(text: str) -> bool:
    lower = normalize_text(text)
    return any(lower.startswith(term) for term in QUERY_TERMS)


def build_review_payload(paths: list[str], *, scan_all: bool, min_score: int) -> dict[str, Any]:
    if ROOT_CHECKER.exists():
        payload = run_root_checker(paths, scan_all=scan_all, min_score=min_score)
    else:
        payload = fallback_review(paths, scan_all=scan_all, min_score=min_score)

    for result in payload["results"]:
        workflow_check = analyze_workflow_module(Path(result["path"]))
        result["workflow_module_check"] = workflow_check
        result["findings"].extend(workflow_check["findings"])
        result["minimal_refactor"].extend(workflow_check["minimal_refactor"])

    payload["summary"]["workflow_module_warnings"] = sum(
        1 for result in payload["results"] if result["workflow_module_check"]["warnings"] > 0
    )
    return payload


def run_root_checker(paths: list[str], *, scan_all: bool, min_score: int) -> dict[str, Any]:
    cmd = [sys.executable, str(ROOT_CHECKER), "--json", "--min-score", str(min_score), "--no-fail"]
    if scan_all:
        cmd.append("--all")
    else:
        cmd.extend(paths)
    completed = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, check=False)
    if completed.returncode == 2:
        raise SystemExit(completed.stderr.strip() or completed.stdout.strip() or "review checker failed")
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"failed to parse root checker JSON output: {exc}") from exc


def fallback_review(paths: list[str], *, scan_all: bool, min_score: int) -> dict[str, Any]:
    candidates = discover_review_targets(paths, scan_all=scan_all)
    results = [fallback_score_file(path, min_score=min_score) for path in candidates]
    summary = {
        "checked": len(results),
        "passed": sum(1 for result in results if result["passed"]),
        "failed": sum(1 for result in results if not result["passed"]),
        "skipped": 0,
        "skipped_paths": [],
        "min_score": min_score,
        "format": "json",
        "all": scan_all,
        "average_score": round(sum(result["score"] for result in results) / len(results), 2) if results else 0.0,
        "scorecard_path": ".agents/skills/shared/use_case_review_scorecard.md",
    }
    return {"summary": summary, "results": results}


def discover_review_targets(paths: list[str], *, scan_all: bool) -> list[Path]:
    if scan_all:
        return sorted(ROOT.glob("lib/**/workflow/**/*.rs"))
    if not paths:
        raise SystemExit("provide at least one path or use --all")

    candidates: list[Path] = []
    for raw_path in paths:
        path = Path(raw_path)
        if not path.is_absolute():
            path = (ROOT / path).resolve()
        if path.is_dir():
            candidates.extend(sorted(path.rglob("*.rs")))
        else:
            candidates.append(path)
    return candidates


def fallback_score_file(path: Path, *, min_score: int) -> dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    use_case_name = first_match(text, r"impl\s+CommandUseCase3\s+for\s+([A-Za-z0-9_]+)") or path.stem
    command_type = first_match(text, r"type\s+Command\s*=\s*([A-Za-z0-9_]+)")
    state_type = first_match(text, r"type\s+GivenState\s*=\s*([A-Za-z0-9_]+)")
    error_type = first_match(text, r"type\s+Error\s*=\s*([A-Za-z0-9_]+)")
    role = first_match(text, r'fn\s+role\(&self\)\s*->\s*&\'static str\s*\{\s*"([^"]+)"')

    score = 100
    findings: list[str] = []
    minimal_refactor: list[str] = []

    if role and any(term in role.lower() for term in TECH_ROLE_TERMS):
        score -= 20
        findings.append(f"`role()` returns technical name `{role}` instead of a clear business role.")
        minimal_refactor.append("Rename `role()` to the business actor that owns the command.")

    if "fn party_id(&self)" not in text:
        score -= 10
        findings.append("The command does not clearly implement `IssuedByParty::party_id()`.")
        minimal_refactor.append("Carry the issuing party on the command and expose it via `party_id()`.")

    if re.search(r"trace_id", text) and re.search(r"stable_entity_id\(&.*trace_id", text):
        score -= 10
        findings.append("`trace_id` appears to be used as business identity.")
        minimal_refactor.append("Use domain identity or `command_id` for idempotency and keep `trace_id` observability-only.")

    if re.search(r"\b(accepted|generated|decision|reply|response|result)\b", text):
        score -= 15
        findings.append("`GivenState` looks partly precomputed, so the use case may be copying answers.")
        minimal_refactor.append("Replace prepared answers in state with domain facts and recompute the business result.")

    if re.search(r"\b(persist|publish|replay|load_state|into_response|json)\b", text):
        score -= 10
        findings.append("The file appears to mix orchestration or transport behavior into the use case boundary.")
        minimal_refactor.append("Keep persistence, publish, replay, load, and reply shaping outside the use case core.")

    score = max(score, 0)
    return {
        "path": str(path.relative_to(ROOT)),
        "use_case_name": use_case_name,
        "command_type": command_type,
        "state_type": state_type,
        "error_type": error_type,
        "role": role,
        "score": score,
        "passed": score >= min_score,
        "category_scores": {"total": score},
        "identity_semantics": {
            "party_id": "detected" if "fn party_id(&self)" in text else "missing",
            "role()": role or "missing",
        },
        "findings": findings or ["No major business-definition issues detected by the fallback heuristic."],
        "minimal_refactor": minimal_refactor or ["No minimal refactor suggested."],
    }


def analyze_workflow_module(path: Path) -> dict[str, Any]:
    if path.is_absolute():
        relative = path.relative_to(ROOT)
    else:
        relative = path
    parts = list(relative.parts)
    module_name = path.parent.name
    use_case_parent = None

    if "workflow" in parts:
        index = parts.index("workflow")
        if index + 1 < len(parts) - 1:
            use_case_parent = parts[index + 1]

    effective_module = use_case_parent or module_name
    findings: list[str] = []
    minimal_refactor: list[str] = []

    if effective_module in GENERIC_MODULE_TERMS:
        findings.append(
            f"Workflow/module boundary is hidden behind technical path segment `{effective_module}` instead of a business workflow module."
        )
        minimal_refactor.append(
            "Introduce a business-named workflow module between `workflow/` and the concrete use case files."
        )

    return {
        "module_name": effective_module,
        "path": str(relative.parent),
        "warnings": len(findings),
        "findings": findings,
        "minimal_refactor": minimal_refactor,
    }


def extract_actions(text: str) -> list[str]:
    explicit = match_list(text, ("use cases", "activities", "steps", "actions"))
    if explicit:
        return [clean_action(action) for action in explicit if clean_action(action)]

    candidates: list[str] = []
    for raw_line in text.splitlines():
        line = raw_line.strip().strip("-*")
        if not line:
            continue
        if re.match(r"^(workflow|process|domain|context|role|actor|performer)\s*:", line, re.IGNORECASE):
            continue
        if re.search(r"\b(" + "|".join(ACTION_TERMS + QUERY_TERMS) + r")\b", normalize_text(line)):
            candidates.append(clean_action(line))

    if candidates:
        return dedupe_preserve_order(candidates)

    sentence_candidates = re.split(r"[。\n.;]+", text)
    return dedupe_preserve_order([clean_action(sentence) for sentence in sentence_candidates if clean_action(sentence)])


def match_value(text: str, labels: tuple[str, ...]) -> str | None:
    for label in labels:
        match = re.search(rf"{re.escape(label)}\s*:\s*(.+)", text, flags=re.IGNORECASE)
        if match:
            return match.group(1).strip()
    return None


def match_list(text: str, labels: tuple[str, ...]) -> list[str]:
    for label in labels:
        match = re.search(rf"{re.escape(label)}\s*:\s*(.+)", text, flags=re.IGNORECASE)
        if match:
            return [part.strip() for part in re.split(r"[,/|]", match.group(1)) if part.strip()]
    return []


def guess_role(actions: list[str]) -> str:
    joined = " ".join(actions).lower()
    if any(token in joined for token in ("deposit", "withdraw", "balance", "account")):
        return "AccountOwner"
    if any(token in joined for token in ("place", "cancel", "trade", "order")):
        return "Trader"
    if any(token in joined for token in ("execute", "commit", "settle")):
        return "Operator"
    return "BusinessActor"


def clean_action(text: str) -> str:
    cleaned = re.sub(r"\s+", " ", text.strip())
    cleaned = re.sub(r"^[0-9]+[.)]\s*", "", cleaned)
    cleaned = re.sub(r"^(when|then|and then)\s+", "", cleaned, flags=re.IGNORECASE)
    return cleaned.strip(" -")


def render_workflow_code(workflow: WorkflowSpec) -> tuple[list[str], dict[str, str]]:
    base = f"{workflow.domain_module}/src/workflow/{workflow.module_name}"
    tree = [
        f"{workflow.domain_module}/",
        "  src/",
        "    workflow/",
        f"      {workflow.module_name}/",
        "        mod.rs",
    ]

    files: dict[str, str] = {
        f"{base}/mod.rs": render_mod_rs(workflow),
    }

    for use_case in workflow.use_cases:
        tree.append(f"        {use_case.file_name}.rs")
        files[f"{base}/{use_case.file_name}.rs"] = render_use_case_rs(workflow, use_case)

    return tree, files


def render_mod_rs(workflow: WorkflowSpec) -> str:
    lines = []
    for use_case in workflow.use_cases:
        lines.append(f"pub mod {use_case.file_name};")
    lines.append("")
    for use_case in workflow.use_cases:
        base = use_case.use_case_name.removesuffix("UseCase")
        lines.append(
            "pub use "
            f"{use_case.file_name}::{{{use_case.command_name}, {use_case.state_name}, "
            f"{use_case.error_name}, {use_case.output_name}, {use_case.reply_name}, {use_case.reply_mapper_name}, "
            f"{use_case.use_case_name}}};"
        )
    return "\n".join(lines) + "\n"


def render_use_case_rs(workflow: WorkflowSpec, use_case: UseCaseSpec) -> str:
    role = workflow.role
    reply_field = "rows" if use_case.kind == "query" else "accepted"
    reply_value = f"result.output.{reply_field}"
    output_value = "0" if use_case.kind == "query" else "true"
    reply_body = "pub rows: usize," if use_case.kind == "query" else "pub accepted: bool,"
    output_body = (
        "pub rows: usize,"
        if use_case.kind == "query"
        else "pub accepted: bool,"
    )

    return f"""use cmd_handler::command_use_case_def2::{{
    CommandUseCase3, IssuedByParty, UseCaseOutput, UseCaseReplyMapper3,
}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum {use_case.error_name} {{
    MissingPartyId,
    InvalidCommand,
    BusinessRuleViolated,
}}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct {use_case.command_name} {{
    pub party_id: String,
}}

impl IssuedByParty for {use_case.command_name} {{
    fn party_id(&self) -> Option<&str> {{
        Some(self.party_id.as_str())
    }}
}}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct {use_case.state_name} {{}}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct {use_case.output_name} {{
    {output_body}
}}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct {use_case.reply_name} {{
    {reply_body}
}}

#[derive(Debug, Clone, Copy, Default)]
pub struct {use_case.reply_mapper_name};

impl UseCaseReplyMapper3 for {use_case.reply_mapper_name} {{
    type Output = {use_case.output_name};
    type Reply = {use_case.reply_name};

    fn map(&self, result: UseCaseOutput<Self::Output>) -> Self::Reply {{
        {use_case.reply_name} {{
            {reply_field}: {reply_value},
        }}
    }}
}}

#[derive(Debug, Clone, Copy, Default)]
pub struct {use_case.use_case_name};

impl CommandUseCase3 for {use_case.use_case_name} {{
    type Command = {use_case.command_name};
    type GivenState = {use_case.state_name};
    type Error = {use_case.error_name};
    type Output = {use_case.output_name};

    fn role(&self) -> &'static str {{
        "{role}"
    }}

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {{
        if cmd.party_id.trim().is_empty() {{
            return Err({use_case.error_name}::MissingPartyId);
        }}
        Ok(())
    }}

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &Self::GivenState,
    ) -> Result<(), Self::Error> {{
        Ok(())
    }}

    fn compute_output_and_events(
        &self,
        _cmd: &Self::Command,
        _state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error> {{
        let output = {use_case.output_name} {{
            {reply_field}: {output_value},
        }};
        Ok(UseCaseOutput {{ output, events: Vec::new() }})
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn role_returns_business_actor() {{
        assert_eq!({use_case.use_case_name}.role(), "{role}");
    }}

    #[test]
    fn pre_check_command_rejects_blank_party_id() {{
        let cmd = {use_case.command_name} {{ party_id: String::new() }};
        let result = {use_case.use_case_name}.pre_check_command(&cmd);
        assert_eq!(result, Err({use_case.error_name}::MissingPartyId));
    }}

    #[test]
    fn validate_against_state_allows_happy_path() {{
        let cmd = {use_case.command_name} {{ party_id: "party-1".to_string() }};
        let state = {use_case.state_name}::default();
        let result = {use_case.use_case_name}.validate_against_state(&cmd, &state);
        assert_eq!(result, Ok(()));
    }}

    #[test]
    fn compute_output_and_events_returns_business_result_and_events() {{
        let cmd = {use_case.command_name} {{ party_id: "party-1".to_string() }};
        let state = {use_case.state_name}::default();
        let result = {use_case.use_case_name}.compute_output_and_events(&cmd, state);
        assert!(matches!(result, Ok(result) if result.events.is_empty()));
    }}

    #[test]
    fn reply_mapper_maps_events_to_reply() {{
        let reply = {use_case.reply_mapper_name}.map(UseCaseOutput {{
            output: {use_case.output_name} {{ {reply_field}: {output_value} }},
            events: Vec::new(),
        }});
        assert_eq!(reply.{reply_field}, {output_value});
    }}

    #[test]
    fn executor_happy_path_is_stubbed_for_follow_up() {{
        // Fill this with CommandUseCaseExecutor3 + stub outbound once the domain state is known.
    }}

    #[test]
    fn executor_rejection_path_is_stubbed_for_follow_up() {{
        // Fill this with CommandUseCaseExecutor3 rejection behavior once the domain state is known.
    }}
}}
"""


def render_generate(payload: GeneratePayload, output_format: str) -> None:
    if output_format == "json":
        print(
            json.dumps(
                {
                    "status": payload.status,
                    "assumptions": payload.assumptions,
                    "questions": [asdict(question) for question in payload.questions],
                    "workflow": asdict(payload.workflow) if payload.workflow else None,
                    "tree": payload.tree,
                    "files": payload.files,
                },
                ensure_ascii=False,
                indent=2,
            )
        )
        return

    if payload.status == "needs_user_choice":
        print("Status: needs_user_choice")
        if payload.assumptions:
            print("\nAssumptions:")
            for assumption in payload.assumptions:
                print(f"- {assumption}")
        print("\nQuestions:")
        for question in payload.questions:
            print(f"- {question.question}")
            print(f"  Recommended: {question.recommended}")
            print(f"  Options: {', '.join(question.options)}")
            print(f"  Impact: {question.impact}")
        return

    print("Status: ready_to_generate")
    if payload.assumptions:
        print("\nAssumptions:")
        for assumption in payload.assumptions:
            print(f"- {assumption}")

    print("\nTree:")
    for line in payload.tree:
        print(line)

    print("\nFiles:")
    for path, content in payload.files.items():
        print(f"\n### {path}")
        print("```rust")
        print(content.rstrip())
        print("```")


def render_review(payload: dict[str, Any], output_format: str) -> None:
    if output_format == "json":
        print(json.dumps(payload, ensure_ascii=False, indent=2))
        return

    summary = payload["summary"]
    print(
        f"Checked: {summary['checked']}, passed: {summary['passed']}, failed: {summary['failed']}, "
        f"workflow warnings: {summary.get('workflow_module_warnings', 0)}"
    )
    for result in payload["results"]:
        print(f"\nPath: {result['path']}")
        print(f"UseCase: {result.get('use_case_name')}")
        print(f"Score: {result['score']}")
        print("Findings:")
        for finding in result["findings"]:
            print(f"- {finding}")
        print("Minimal Refactor:")
        for item in result["minimal_refactor"]:
            print(f"- {item}")


def normalize_text(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", " ", value.lower()).strip()


def to_snake_case(value: str) -> str:
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    parts = [part for part in re.split(r"[^A-Za-z0-9]+", value) if part]
    return "_".join(part.lower() for part in parts) or "workflow_module"


def to_pascal_case(value: str, *, suffix: str = "") -> str:
    parts = [part.capitalize() for part in re.split(r"[^A-Za-z0-9]+", value) if part]
    base = "".join(parts) or "Generated"
    if suffix and not base.endswith(suffix):
        base += suffix
    return base


def sanitize_module_name(value: str) -> str:
    lowered = to_snake_case(value)
    lowered = re.sub(r"(_workflow|_process)$", "", lowered)
    return lowered or "workflow_module"


def titleize_role(value: str) -> str:
    return to_pascal_case(value)


def dedupe_preserve_order(items: list[str]) -> list[str]:
    seen: set[str] = set()
    deduped: list[str] = []
    for item in items:
        key = item.lower()
        if key in seen:
            continue
        seen.add(key)
        deduped.append(item)
    return deduped


def first_match(text: str, pattern: str) -> str | None:
    match = re.search(pattern, text, flags=re.MULTILINE)
    return match.group(1) if match else None


if __name__ == "__main__":
    raise SystemExit(main())
