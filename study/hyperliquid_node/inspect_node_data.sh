#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
printf '%s\n' '== data directories ==' 
find runtime/hl -maxdepth 2 -type d -print | sort
printf '%s\n' '== largest files ==' 
find runtime/hl -type f -print0 2>/dev/null | xargs -0 du -h 2>/dev/null | sort -hr | head -20 || true
printf '%s\n' '== recent writes ==' 
find runtime/hl -type f -print0 2>/dev/null | xargs -0 stat -f '%Sm %N' -t '%Y-%m-%dT%H:%M:%S' 2>/dev/null | sort -r | head -20 || true
