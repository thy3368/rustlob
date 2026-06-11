#!/usr/bin/env python3
"""Compatibility wrapper for the RustLOB use case definition checker."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
WORKFLOW_TOOL = (
    ROOT
    / ".agents"
    / "skills"
    / "workflow-use-case-modeling"
    / "scripts"
    / "workflow_use_case_tool.py"
)


def main() -> int:
    cmd = [sys.executable, str(WORKFLOW_TOOL), "review", *sys.argv[1:]]
    completed = subprocess.run(cmd, cwd=ROOT, check=False)
    return completed.returncode


if __name__ == "__main__":
    raise SystemExit(main())
