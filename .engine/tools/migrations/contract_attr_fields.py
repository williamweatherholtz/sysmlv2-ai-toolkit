#!/usr/bin/env python3
"""One-time migration (D0063 CONTRACT step, attrModelContract): drop the deprecated
attribution fields now that the new ones are populated everywhere and the readers prefer them.

- `authoredBy` assignment is removed ONLY where a `createdBy` assignment immediately follows
  (the populate step put them inline), so no instance is ever left without a creator.
- `decisionText` assignment LINES are removed only in .engine/decisions/ files, all of which
  now carry `decision` + `rationale` (the split is complete).

Schema attribute DEFINITIONS are edited by hand (this script only touches instance `:>>`).

Run from repo root:
  python .engine/tools/migrations/contract_attr_fields.py            # dry-run (counts only)
  python .engine/tools/migrations/contract_attr_fields.py --apply    # write
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]

# `:>> authoredBy = "V";` (+ trailing spaces) ONLY when a createdBy assignment follows.
_AUTHORED = re.compile(r':>>\s*authoredBy\s*=\s*"[^"]*"\s*;[ \t]*(?=:>>\s*createdBy)')
# A whole `:>> decisionText = "..."[ + "..."]* ;` assignment — single-line OR multi-line
# string concatenation (early decisions wrap long values as `"frag" + "frag" + ...;`).
_DECTEXT = re.compile(
    r'(?m)^[ \t]*:>>\s*decisionText\s*=\s*"[^"]*"(?:\s*\+\s*"[^"]*")*\s*;[ \t]*\r?\n'
)


def strip_text(text: str, *, is_decision: bool) -> tuple[str, int, int]:
    text, n_auth = _AUTHORED.subn("", text)
    n_dec = 0
    if is_decision:
        text, n_dec = _DECTEXT.subn("", text)
    return text, n_auth, n_dec


def main() -> int:
    apply = "--apply" in sys.argv
    roots = [ROOT / ".tracking", ROOT / ".engine"]
    skip = ROOT / ".engine" / "schema"
    decisions_dir = ROOT / ".engine" / "decisions"

    files = tot_auth = tot_dec = 0
    for base in roots:
        for path in base.rglob("*.sysml"):
            if str(path).startswith(str(skip)):
                continue
            original = path.read_text(encoding="utf-8")
            new, n_auth, n_dec = strip_text(original, is_decision=(decisions_dir in path.parents))
            if n_auth or n_dec:
                files += 1
                tot_auth += n_auth
                tot_dec += n_dec
                if apply:
                    path.write_text(new, encoding="utf-8")
                print(f"  {path.relative_to(ROOT)}: -{n_auth} authoredBy, -{n_dec} decisionText")

    verb = "stripped" if apply else "would strip"
    print(f"\ncontract_attr_fields ({'APPLY' if apply else 'DRY-RUN'}): {verb} "
          f"{tot_auth} authoredBy + {tot_dec} decisionText across {files} file(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
