#!/usr/bin/env python3
# not-an-instrument: a one-time D0067 transform over seven process files; it measures nothing.
"""D0067 MIGRATE step for D0321 option A / D0434 (2026-09-10): a ProcessStep declares the check that checks it.

Before: a process was a document with an optional guard. Only agile-workflow's ceremony steps carried a
per-step check, hand-coded in `keel advance`; the seven processes D0321 measured as bindable had their
guard in prose and in `process-enforcement.toml`, and nothing joined a STEP to the predicate that checks it.

After (this transform, applied once): seven steps carry `:>> checkedBy = "<guard>";` -
the guard's GUARD_NAMES entry - written as the step's last attribute, after `owner`:

  * control-defect      cd4Register          control-defect-registry   (guard 57 is its register step's predicate)
  * decision-surfacing  dsGround             judgment-request-quality  (guard 48 checks the fork's shape)
  * library-stewardship lsStock              unit-extras-present       (guard 60)
  * stpa-diagram        sd0Compute           viewpoint-renderer        (guard 12: the renderer exists)
  * stpa-self           stpa5Trigger         stpa-currency             (guard 62)
  * github-intake       giRoute              untrusted-routing         (guard 56)
  * github-intake       giBoundary           untrusted-taint           (guard 63)

D0321 named an eighth - migration's mig2DryRunReconcile to `marker-census` - and the guard's first live run
refused it: marker-census is a LENS (commands.sysml, family lens), not a guard, so nothing runs it on the
step; the migration process stays unbound until a guard reads its control totals (dcMigrationTotalsAreAGuard).

Every other step stays unbound and valid: `checkedBy` is `[0..1]` (the EXPAND, in process.sysml), so this
run contracts nothing. SECOND RUN (D0435, the same day): the six agile-workflow ceremony steps bind the
per-run gate that records them - refineLink gate:refine, standupGate gate:standup, implDo gate:implement,
implReview gate:review, implClose gate:closeOut, retroIdentify gate:retro - now that `keel advance` reads
the binding (the first run left them for their reader: the D0144 producer-before-consumer shape).

Control totals (gate 2 of the `migration` skill) - the run FAILS before writing when any does not balance:

  * conservation:  top-level processes and ProcessStep declarations are counted over EVERY process file
                   before and after and must be equal (38 processes, 185 steps on the tree this was written
                   against; the script reads the live numbers and prints them, it does not assume them);
  * no-leak:       bound + already-bound + unbound == steps, and every planned site is found exactly once
                   (a step name found 0 or 2+ times refuses the run);
  * content hash:  SHA-256 over every OTHER line of the seven files is equal before and after, so the
                   transform proves that nothing it did not name moved.

Idempotent: a site already carrying `checkedBy` is skipped and counted as such; re-running on the
migrated tree reports 0 changes and the same totals. Line endings are preserved per file. Dry run is the
default; `--apply` writes.

Run from the repository root:

    python .engine/tools/migrations/2026-09-10-step-checked-by.py            # dry run + reconcile
    python .engine/tools/migrations/2026-09-10-step-checked-by.py --apply    # write, then re-reconcile
"""
from __future__ import annotations

import hashlib
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PROCESSES = ROOT / ".engine" / "processes"

# (file stem, step name, check) - the seven of D0321's eight bindings that name a guard, then D0435's six gates.
BINDINGS: list[tuple[str, str, str]] = [
    ("control-defect", "cd4Register", "control-defect-registry"),
    ("decision-surfacing", "dsGround", "judgment-request-quality"),
    ("library-stewardship", "lsStock", "unit-extras-present"),
    ("stpa-diagram", "sd0Compute", "viewpoint-renderer"),
    ("stpa-self", "stpa5Trigger", "stpa-currency"),
    ("github-intake", "giRoute", "untrusted-routing"),
    ("github-intake", "giBoundary", "untrusted-taint"),
    # D0435 (second run, same day): the ceremony steps name the per-run gate that records them.
    ("agile-workflow", "refineLink", "gate:refine"),
    ("agile-workflow", "standupGate", "gate:standup"),
    ("agile-workflow", "implDo", "gate:implement"),
    ("agile-workflow", "implReview", "gate:review"),
    ("agile-workflow", "implClose", "gate:closeOut"),
    ("agile-workflow", "retroIdentify", "gate:retro"),
]

STEP_RE = re.compile(r"^\s*action\s+(\w+)\s*:\s*ProcessStep\b")
PROCESS_RE = re.compile(r"^\s*action\s+(\w+)\s*:\s*Process\b")  # `\b` keeps ProcessStep out (hardening.rs:313)


def read(path: Path) -> tuple[list[str], str]:
    """Lines WITHOUT their ending, plus the file's dominant line ending, so a CRLF file stays CRLF."""
    raw = path.read_bytes().decode("utf-8")
    ending = "\r\n" if raw.count("\r\n") >= raw.count("\n") - raw.count("\r\n") and "\r\n" in raw else "\n"
    return raw.split(ending), ending


def totals() -> tuple[int, int, int]:
    """(processes, steps, steps carrying checkedBy) over every process file."""
    procs = steps = bound = 0
    for f in sorted(PROCESSES.glob("*.sysml")):
        text = f.read_text(encoding="utf-8")
        for line in text.splitlines():
            if PROCESS_RE.match(line):
                procs += 1
            elif STEP_RE.match(line):
                steps += 1
        bound += text.count(":>> checkedBy = ")
    return procs, steps, bound


def step_block(lines: list[str], name: str) -> tuple[int, int]:
    """(start, end) line indices of `action <name> : ProcessStep { ... }`; refuses 0 or 2+ matches."""
    starts = [i for i, l in enumerate(lines) if STEP_RE.match(l) and STEP_RE.match(l).group(1) == name]
    if len(starts) != 1:
        raise SystemExit(f"REFUSED: step {name} found {len(starts)} times (need exactly 1)")
    i = starts[0]
    depth = 0
    for j in range(i, len(lines)):
        depth += lines[j].count("{") - lines[j].count("}")
        if depth == 0 and j > i:
            return i, j
    raise SystemExit(f"REFUSED: step {name} block never closes")


def other_lines_hash(files: dict[str, list[str]], skip: set[tuple[str, int]]) -> str:
    h = hashlib.sha256()
    for stem in sorted(files):
        for i, line in enumerate(files[stem]):
            if (stem, i) not in skip:
                h.update(line.encode("utf-8"))
                h.update(b"\n")
    return h.hexdigest()


def main(apply: bool) -> int:
    before = totals()
    print(f"before: processes={before[0]} steps={before[1]} bound={before[2]}")

    files: dict[str, tuple[list[str], str]] = {}
    for stem in sorted({b[0] for b in BINDINGS}):
        files[stem] = read(PROCESSES / f"{stem}.sysml")

    planned: list[tuple[str, int, str]] = []  # (stem, insert-after index, line)
    already = 0
    for stem, step, guard in BINDINGS:
        lines, _ = files[stem]
        s, e = step_block(lines, step)
        block = lines[s : e + 1]
        if any(":>> checkedBy = " in l for l in block):
            already += 1
            continue
        owner = [k for k in range(s, e + 1) if ":>> owner = " in lines[k]]
        if len(owner) != 1:
            raise SystemExit(f"REFUSED: step {step} has {len(owner)} owner lines (need exactly 1)")
        indent = lines[owner[0]][: len(lines[owner[0]]) - len(lines[owner[0]].lstrip())]
        planned.append((stem, owner[0], f'{indent}:>> checkedBy = "{guard}";'))
        print(f"  bind {stem}:{step} -> {guard}")

    # Content hash of every line the transform does not touch, before and after (computed on the
    # projected output so the dry run proves the same thing the apply does).
    orig = {stem: lines for stem, (lines, _) in files.items()}
    before_hash = other_lines_hash(orig, set())
    projected: dict[str, list[str]] = {stem: list(lines) for stem, lines in orig.items()}
    inserted: set[tuple[str, int]] = set()
    for stem in projected:
        # insert from the bottom up so earlier indices stay valid
        for _, after, line in sorted((p for p in planned if p[0] == stem), key=lambda p: -p[1]):
            projected[stem].insert(after + 1, line)
    # The inserted lines are the ones at a `checkedBy` position that the original lacked at that index.
    for stem in projected:
        j = 0
        for i, line in enumerate(projected[stem]):
            if j < len(orig[stem]) and line == orig[stem][j]:
                j += 1
            else:
                inserted.add((stem, i))
    after_hash = other_lines_hash(projected, inserted)
    if before_hash != after_hash:
        raise SystemExit("REFUSED: content hash of untouched lines differs on the projected output")
    if len(inserted) != len(planned):
        raise SystemExit(f"REFUSED: {len(inserted)} inserted lines vs {len(planned)} planned")

    expected_bound = before[2] + len(planned)
    print(f"plan: bind={len(planned)} already-bound={already} unbound-after={before[1] - expected_bound}")
    if len(planned) + already != len(BINDINGS):
        raise SystemExit("REFUSED: no-leak - planned + already != bindings")
    print(f"content hash (untouched lines): {before_hash[:16]} == {after_hash[:16]}")

    if not apply:
        print("dry run - nothing written (--apply to write)")
        return 0

    for stem, (lines, ending) in files.items():
        (PROCESSES / f"{stem}.sysml").write_bytes(ending.join(projected[stem]).encode("utf-8"))
    after = totals()
    print(f"after:  processes={after[0]} steps={after[1]} bound={after[2]}")
    if after[0] != before[0] or after[1] != before[1]:
        raise SystemExit("FAILED: conservation - process or step count moved")
    if after[2] != expected_bound:
        raise SystemExit(f"FAILED: bound {after[2]} != expected {expected_bound}")
    print("applied and reconciled")
    return 0


if __name__ == "__main__":
    sys.exit(main("--apply" in sys.argv[1:]))
