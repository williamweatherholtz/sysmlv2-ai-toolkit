#!/usr/bin/env python3
"""Bind the `safety` viewpoint to its renderer: `keel show control-structure` (D0284, st066).

The viewpoint registry declares `safetyVP` with `renderer = "(planned) STPA safety view ... not yet
rendered"`, and guard `viewpoint-renderer` has warned on it since the registry was written. The
renderer now exists: `keel show control-structure` computes STPA step 2 for the project's own workflow
from the hook config, git hooks, workflow files, CLI facts and declared deciders. This transform
rewrites ONE field on ONE item.

WHY A D0067 TRANSFORM FOR ONE FIELD. The registry is owned by the human (`createdBy = wweatherholtz`),
and D0108 forbids a non-owner overwriting another actor's field in place. The `ownership` guard's
sanctioned exemption is a co-committed transform under `.engine/tools/migrations/` - the same path the
2026-09-01 lens rewrite used for ten renderer fields. The transform is the record, in the commit, of
why the boundary was crossed and what exactly changed: a reviewer can re-run it and diff.

Idempotent: a second run finds nothing to change and says so. Dry run by default; `--apply` writes.
"""
import io, sys, pathlib

ROOT = pathlib.Path(__file__).resolve().parents[3]
REG = ROOT / ".engine" / "views" / "viewpoint-registry.sysml"
OLD = ':>> renderer = "(planned) STPA safety view (losses/hazards/UCAs/constraints + verify coverage) — not yet rendered";'
NEW = ':>> renderer = "keel show control-structure (STPA step 2, computed; losses/hazards from engine-safety.sysml via hazardsByProcess)";'

def main() -> int:
    apply = "--apply" in sys.argv
    text = io.open(REG, encoding="utf-8", newline="").read()
    n = text.count(OLD)
    if n == 0:
        print(f"nothing to change: safetyVP renderer already bound ({REG.relative_to(ROOT)})")
        return 0
    if n != 1:
        print(f"REFUSED: expected exactly one planned safety renderer, found {n}")
        return 2
    print(f"{'APPLY' if apply else 'DRY RUN'}: 1 field in {REG.relative_to(ROOT)}\n  - {OLD}\n  + {NEW}")
    if apply:
        io.open(REG, "w", encoding="utf-8", newline="").write(text.replace(OLD, NEW, 1))
        print("written")
    return 0

if __name__ == "__main__":
    sys.exit(main())
