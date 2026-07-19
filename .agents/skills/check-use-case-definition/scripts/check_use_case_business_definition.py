#!/usr/bin/env python3
"""Local wrapper for the RustLOB use case definition checker."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
CHECKER = ROOT / "scripts" / "check_use_case_business_definition.py"


def main() -> int:
    cmd = [sys.executable, str(CHECKER), *sys.argv[1:]]
    completed = subprocess.run(cmd, cwd=ROOT, check=False)
    return completed.returncode


if __name__ == "__main__":
    raise SystemExit(main())
