#!/usr/bin/env python3
# not-an-instrument: a one-time D0067 transform over eighteen decision files; it measures nothing.
"""D0067 MIGRATE step for D0384 option A / D0398 (2026-09-09): supersession is said ONCE, by the edge.

Before: a Decision's retirement was writable in two places - `status = DecisionStatus::superseded` and an
incoming `#Supersede` edge - and nothing held them equal (issue396: d0353 sat proposed on the queue for a
day after d0356 retired it). Ten `#Supersede` edges targeted a Decision, and seven of them reversed ONE
clause of a target that stayed accepted and in force (d0149 -> d0129 leaves never-rebase standing), while
every Rust reader of `kind == supersede` treated the target as retired whole.

After (this transform, applied once):

  * the SEVEN clause reversals are retyped `#SupersedeClause` (D0398: reverses one clause, target in force);
  * the FOUR retirements recorded only in a trailing comment (d0003 by D0096, d0038 by D0044, d0069 by D0070,
    d0071 by D0075 - each superseder's own text names the target) become `#Supersede` edges authored in the
    superseder's file, with `private import EngineRelationships::*;` added where the file lacks it;
  * the SEVEN `status = DecisionStatus::superseded` values take the standing the Decision had when retired -
    `accepted` where a human acceptance result (`dNNNNAcceptR1`) is on record in the file, `proposed` where
    none is (d0038 was never accepted; d0353 was proposed and hand-flipped at issue396). Nothing is invented:
    the basis for every value is a fact already in the same file.

The three edges that already retire a target reading `superseded` (d0129 -> d0014, d0311 -> d0310,
d0356 -> d0353) stay `#Supersede`. The 45 `#Supersede` edges to Needs / requirements / tasks / releases are
out of scope: they retire whole targets and keep their meaning.

Control totals (gate 2 of the `migration` skill) - the run FAILS before writing when any does not balance:

  * conservation:  superseded-status sites 7 -> 0; Decision-target edges 10 -> 14 = 3 whole + 7 clause + 4 new;
  * no-leak:       every planned site is found exactly once (a site found 0 or 2+ times refuses the run);
  * content hash:  SHA-256 over every OTHER line of the eighteen files is equal before and after, so the
                   transform proves that nothing it did not name moved.

Idempotent: a site already in the new shape is skipped and counted as such; re-running on the migrated tree
reports 0 changes and the same totals. Line endings are preserved per file (two of these files are CRLF in
a Windows checkout). Dry run is the default; `--apply` writes.

Run from the repository root:

    python .engine/tools/migrations/2026-09-09-supersede-scope.py            # dry run + reconcile
    python .engine/tools/migrations/2026-09-09-supersede-scope.py --apply    # write, then re-reconcile
"""
from __future__ import annotations

import hashlib
import io
import os
import sys
import tempfile
from pathlib import Path

DECISIONS = Path(".engine") / "decisions"

# --- the plan: every site named, with its recorded basis -------------------------------------------------

# status = superseded -> the standing the Decision had when retired. Basis: an acceptance result in the file.
STATUS_SITES = ["d0003", "d0014", "d0038", "d0069", "d0071", "d0310", "d0353"]

# #Supersede -> #SupersedeClause: the source reverses one clause of a target that stays in force.
CLAUSE_EDGES = [
    ("d0152", "d0114"),
    ("d0158", "d0115"),
    ("d0149", "d0129"),
    ("d0251", "d0175"),
    ("d0289", "d0178"),
    ("d0291", "d0205"),
    ("d0359", "d0351"),
]

# Retirements recorded only in prose: (superseder, target, the line in the superseder's file that records it).
NEW_EDGES = [
    ("d0096", "d0003", "Decision-supersedes-decision: d0003.status -> superseded"),
    ("d0044", "d0038", "superseded by D0044"),  # recorded in d0038's own trailing comment; d0044 raises the cap it replaced
    ("d0070", "d0069", "Supersedes D0069"),
    ("d0075", "d0071", "SUPERSEDES D0071"),
]

# Edges that already retire a target reading `superseded` - untouched, counted.
WHOLE_EDGES = [("d0129", "d0014"), ("d0311", "d0310"), ("d0356", "d0353")]

IMPORT_LINE = "    private import EngineRelationships::*;"


def decision_file(d: str) -> Path:
    nnnn = d[1:]
    hits = sorted(DECISIONS.glob(f"{nnnn}-*.sysml"))
    if len(hits) != 1:
        sys.exit(f"REFUSED: {len(hits)} files for {d} under {DECISIONS} - expected exactly one")
    return hits[0]


def read(path: Path) -> tuple[list[str], str]:
    """The file's lines without terminators, and the terminator it uses ('\\r\\n' or '\\n')."""
    raw = io.open(path, encoding="utf-8", newline="").read()
    eol = "\r\n" if "\r\n" in raw else "\n"
    body = raw.replace("\r\n", "\n")
    lines = body.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    return lines, eol


def write_atomically(path: Path, lines: list[str], eol: str) -> None:
    text = eol.join(lines) + eol
    fd, tmp = tempfile.mkstemp(prefix=".supersede-scope-", dir=str(path.parent))
    with io.open(fd, "w", encoding="utf-8", newline="") as f:
        f.write(text)
    os.replace(tmp, path)


def has_acceptance(lines: list[str], d: str) -> bool:
    return any(f"{d}AcceptR" in ln and ": TestResult" in ln for ln in lines)


def edge_line(marker: str, src: str, dst: str) -> str:
    return f"    #{marker} dependency from {src} to {dst};"


# --- the transform over ONE loaded tree -----------------------------------------------------------------

class Plan:
    def __init__(self) -> None:
        self.files: dict[Path, tuple[list[str], str]] = {}
        self.changed = 0
        self.skipped = 0
        self.touched_lines: dict[Path, set[int]] = {}

    def load(self, d: str) -> tuple[Path, list[str]]:
        p = decision_file(d)
        if p not in self.files:
            self.files[p] = read(p)
            self.touched_lines[p] = set()
        return p, self.files[p][0]

    def touch(self, p: Path, idx: int) -> None:
        self.touched_lines[p].add(idx)

    def only_index(self, p: Path, lines: list[str], pred, what: str) -> int:
        idx = [i for i, ln in enumerate(lines) if pred(ln)]
        if len(idx) != 1:
            sys.exit(f"REFUSED (no-leak): {what} occurs {len(idx)} times in {p} - expected exactly once")
        return idx[0]

    def migrate_status(self, d: str) -> None:
        p, lines = self.load(d)
        old = f"status = DecisionStatus::superseded"
        idx = [i for i, ln in enumerate(lines) if old in ln]
        if not idx:
            # already migrated: the line must read accepted or proposed, once
            i = self.only_index(p, lines, lambda ln: ":>> status = DecisionStatus::" in ln, f"{d} status line")
            self.touch(p, i)
            self.skipped += 1
            return
        if len(idx) != 1:
            sys.exit(f"REFUSED (no-leak): {d} carries {len(idx)} superseded status lines")
        i = idx[0]
        standing = "accepted" if has_acceptance(lines, d) else "proposed"
        lines[i] = lines[i].replace(old, f"status = DecisionStatus::{standing}", 1)
        self.touch(p, i)
        self.changed += 1
        print(f"  {p.name}: status superseded -> {standing} ({'acceptance result on record' if standing == 'accepted' else 'no acceptance on record'})")

    def retype_edge(self, src: str, dst: str) -> None:
        p, lines = self.load(src)
        old, new = edge_line("Supersede", src, dst), edge_line("SupersedeClause", src, dst)
        i = self.only_index(p, lines, lambda ln: ln.rstrip() in (old, new), f"edge {src} -> {dst}")
        self.touch(p, i)
        if lines[i].rstrip() == new:
            self.skipped += 1
            return
        lines[i] = new
        self.changed += 1
        print(f"  {p.name}: #Supersede -> #SupersedeClause ({src} -> {dst})")

    def author_edge(self, src: str, dst: str, basis: str) -> None:
        p, lines = self.load(src)
        if not any(basis in ln for ln in lines) and not any(basis in ln for ln in self.load(dst)[1]):
            sys.exit(f"REFUSED (basis): neither {src} nor {dst} records {basis!r} - an edge with no recorded basis is fabricated")
        new = edge_line("Supersede", src, dst)
        if any(ln.rstrip() == new for ln in lines):
            i = self.only_index(p, lines, lambda ln: ln.rstrip() == new, f"edge {src} -> {dst}")
            self.touch(p, i)
            if not any(ln.rstrip() == IMPORT_LINE.rstrip() for ln in lines):
                sys.exit(f"REFUSED: {p} carries the edge but not {IMPORT_LINE.strip()}")
            for j, ln in enumerate(lines):
                if ln.rstrip() == IMPORT_LINE.rstrip() or (basis in ln and "D0398's transform" in ln):
                    self.touch(p, j)
            self.skipped += 1
            return
        # the package's closing brace is the last non-empty line
        close = max(i for i, ln in enumerate(lines) if ln.strip() == "}")
        if not any(ln.rstrip() == IMPORT_LINE.rstrip() for ln in lines):
            last_import = max(i for i, ln in enumerate(lines) if ln.strip().startswith("private import "))
            lines.insert(last_import + 1, IMPORT_LINE)
            close += 1
            print(f"  {p.name}: + {IMPORT_LINE.strip()}")
        lines.insert(close, f"    // {basis} (recorded in prose; the edge authored 2026-09-09 by D0398's transform - the edge is the fact)")
        lines.insert(close + 1, new)
        # every inserted line is the plan's own: the import, the basis comment and the edge
        for i, ln in enumerate(lines):
            if ln.rstrip() in (IMPORT_LINE.rstrip(), new) or (basis in ln and "D0398's transform" in ln):
                self.touch(p, i)
        self.changed += 1
        print(f"  {p.name}: + #Supersede {src} -> {dst}  (basis: {basis!r})")

    def content_hash(self) -> str:
        """SHA-256 over the UNNAMED lines of every loaded file, as the plan holds them (path order)."""
        return unnamed_hash({p: lines for p, (lines, _) in self.files.items()})


def is_named(line: str, declares_status_site: bool) -> bool:
    """Is this line of one of the four classes the transform names? Everything else must not move.

    Named: the `status` line of a Decision this plan re-standings; any Decision-to-Decision `#Supersede` /
    `#SupersedeClause` edge; the EngineRelationships import; the basis comment the transform writes. Named
    lines are dropped from BOTH sides of the hash by content, so an authored line shifting the lines
    below it is not a change - the lines below keep their text and their relative order.
    """
    s = line.strip()
    return (
        (s.startswith(":>> status = DecisionStatus::") and declares_status_site)
        or (s.startswith("#Supersede") and " dependency from d0" in s and " to d0" in s)
        or s == IMPORT_LINE.strip()
        or "D0398's transform" in s
    )


def unnamed_hash(files: dict[Path, list[str]]) -> str:
    """SHA-256 over every line `is_named` does not claim, in every file, in path order."""
    h = hashlib.sha256()
    for p in sorted(files):
        lines = files[p]
        declares = any(f"part {d} : Decision" in x for d in STATUS_SITES for x in lines)
        for ln in lines:
            if is_named(ln, declares):
                continue
            h.update(ln.encode("utf-8"))
            h.update(b"\n")
    return h.hexdigest()


def disk_hash(paths: list[Path]) -> str:
    """The same hash over the files as they are ON DISK."""
    return unnamed_hash({p: read(p)[0] for p in paths})


def totals(plan: Plan) -> dict[str, int]:
    status_superseded = 0
    whole = clause = 0
    for p, (lines, _) in plan.files.items():
        for ln in lines:
            s = ln.strip()
            if "status = DecisionStatus::superseded" in s:
                status_superseded += 1
            if s.startswith("#Supersede dependency from d0") and " to d0" in s:
                whole += 1
            if s.startswith("#SupersedeClause dependency from d0") and " to d0" in s:
                clause += 1
    # the three whole edges live in files this plan loads only if they are status sites' superseders;
    # count them on disk so the total is over the tree, not the loaded subset
    for src, dst in WHOLE_EDGES:
        p = decision_file(src)
        if p not in plan.files:
            lines, _ = read(p)
            whole += sum(1 for ln in lines if ln.strip() == edge_line("Supersede", src, dst).strip())
    return {"status_superseded": status_superseded, "edges_whole": whole, "edges_clause": clause}


def tree_totals() -> dict[str, int]:
    """The same three counts over EVERY decision file on disk - what the plan must agree with."""
    status_superseded = whole = clause = 0
    for p in sorted(DECISIONS.glob("*.sysml")):
        lines, _ = read(p)
        for ln in lines:
            s = ln.strip()
            if s.startswith(":>> status = DecisionStatus::superseded"):
                status_superseded += 1
            if s.startswith("#Supersede dependency from d0") and " to d0" in s:
                whole += 1
            if s.startswith("#SupersedeClause dependency from d0") and " to d0" in s:
                clause += 1
    return {"status_superseded": status_superseded, "edges_whole": whole, "edges_clause": clause}


def main() -> int:
    apply = "--apply" in sys.argv[1:]
    if not DECISIONS.is_dir():
        sys.exit(f"REFUSED: run from the repository root ({DECISIONS} not found)")

    before = tree_totals()
    print(f"before: {before}")

    plan = Plan()
    for d in STATUS_SITES:
        plan.migrate_status(d)
    for src, dst in CLAUSE_EDGES:
        plan.retype_edge(src, dst)
    for src, dst, basis in NEW_EDGES:
        plan.author_edge(src, dst, basis)

    # --- reconcile (gate 2): the projected tree ------------------------------------------------------
    expected = {"status_superseded": 0, "edges_whole": len(WHOLE_EDGES) + len(NEW_EDGES), "edges_clause": len(CLAUSE_EDGES)}
    got = totals(plan)
    print(f"projected: {got}   expected: {expected}")
    if got != expected:
        sys.exit(f"REFUSED (conservation): projected totals {got} != expected {expected}; nothing written")
    if plan.changed + plan.skipped != len(STATUS_SITES) + len(CLAUSE_EDGES) + len(NEW_EDGES):
        sys.exit(f"REFUSED (no-leak): changed {plan.changed} + skipped {plan.skipped} != planned sites")

    # content hash: the unnamed lines of the on-disk files == the unnamed lines of the projected files
    disk_h = disk_hash(list(plan.files))
    proj_h = plan.content_hash()
    print(f"content hash (unnamed lines): disk {disk_h[:16]}  projected {proj_h[:16]}  {'EQUAL' if disk_h == proj_h else 'DIFFER'}")
    if disk_h != proj_h:
        sys.exit("REFUSED (content): a line the transform did not name would move; nothing written")

    print(f"sites: {plan.changed} to change, {plan.skipped} already in the new shape")
    if not apply:
        print("dry run - pass --apply to write")
        return 0
    for p, (lines, eol) in plan.files.items():
        write_atomically(p, lines, eol)
    after = tree_totals()
    print(f"after: {after}")
    if after != expected:
        sys.exit(f"FAILED post-reconcile: tree totals {after} != expected {expected}")
    print("applied; post-reconcile EQUAL")
    return 0


if __name__ == "__main__":
    sys.exit(main())
