#!/usr/bin/env python3
"""One-time migration (D0063 MIGRATE step, attrModelInstances): populate the new
`createdBy` field from `authoredBy` on every tracked instance.

Expand/migrate/contract: `createdBy` was ADDED additively ([0..1], D0063 expand).
This populates it = the existing `authoredBy` value, on every `:>> authoredBy = "X";`
redefinition, leaving `authoredBy` in place (the contract step removes it later).

Idempotent: skips a site whose `authoredBy` is already immediately followed by a
`createdBy` assignment. Targets `.tracking/**` and `.engine/**` instance files only —
schema attribute DEFINITIONS (`attribute authoredBy : String;`) have no `:>>` and are
never matched, so schema/core is untouched.

Run from repo root:  python .engine/tools/migrations/populate_created_by.py
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

# `:>> authoredBy = "VALUE" ;`  not already followed by a createdBy assignment.
PATTERN = re.compile(
    r'(:>>\s*authoredBy\s*=\s*"([^"]*)"\s*;)(?!\s*:>>\s*createdBy)'
)


def migrate_text(text: str) -> tuple[str, int]:
    """Return (new_text, sites_changed)."""
    count = 0

    def repl(m: re.Match) -> str:
        nonlocal count
        count += 1
        return f'{m.group(1)} :>> createdBy = "{m.group(2)}";'

    return PATTERN.sub(repl, text), count


def main() -> int:
    root = Path(__file__).resolve().parents[3]  # repo root (.engine/tools/migrations/ -> 3 up)
    roots = [root / ".tracking", root / ".engine"]
    # Never rewrite the schema definitions themselves.
    skip_dirs = {root / ".engine" / "schema"}

    total_files = 0
    total_sites = 0
    for base in roots:
        for path in base.rglob("*.sysml"):
            if any(str(path).startswith(str(s)) for s in skip_dirs):
                continue
            original = path.read_text(encoding="utf-8")
            new, n = migrate_text(original)
            if n:
                path.write_text(new, encoding="utf-8")
                total_files += 1
                total_sites += n
                print(f"  {path.relative_to(root)}: +{n} createdBy")

    print(f"\npopulate_created_by: {total_sites} site(s) across {total_files} file(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
