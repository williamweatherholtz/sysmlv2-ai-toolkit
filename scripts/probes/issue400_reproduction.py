#!/usr/bin/env python3
# ci-probe: --probe
# not-an-instrument: a reproduction of two DEFECTIVE checks against their known cases (issue400 / D0388).
# Its numbers are the defects', kept as evidence that a probe catches them; nothing reads them as a measure.
"""issue400: two hand-written checks answered wrong on their first run, and both answers were stated.

  (1) A census of which enforced guards have a test constructing their defect scored 64 of 64 - it searched
      a text window around each guard's NAME across every .rs file for the words FAIL / violation, and every
      guard name occurs in the GUARD_NAMES registry inside guards.rs, which is full of both words. It was
      measuring its own registry. The real answer, from a version that reads test function bodies, was 16.
  (2) A check for controllers lacking a process model reported the HUMAN's missing - it compared the role
      name to the anchor name case-sensitively, `human` against `ctHuman`, so every anchor read as absent.

This script re-runs BOTH defective checks and, for each, a probe: one case whose answer is known to be
positive and one known to be negative, chosen BEFORE the real tree is read. The defective check fails its
probe; the corrected check passes it. Run from the repository root:

    python scripts/probes/issue400_reproduction.py

The practice this evidences (CLAUDE.md section 4, D0388): a check written to answer a question is probed
against a known-positive and a known-negative case before its answer is stated anywhere.
"""
from __future__ import annotations

import glob
import json
import os
import re
import subprocess
import sys

REPO = os.getcwd()
FAILS = 0


def verdict(name: str, ok: bool, detail: str) -> None:
    global FAILS
    print(f"  {'pass' if ok else 'FAIL'}  {name} - {detail}")
    if not ok:
        FAILS += 1


# ------------------------------------------------------------------ (1) the guard-test census
def guard_names() -> list[str]:
    src = open(os.path.join(REPO, "keel-cli", "src", "guards.rs"), encoding="utf-8").read()
    m = re.search(r"pub const GUARD_NAMES: \[&str; \d+\] =\s*\[([^\]]+)\];", src)
    return re.findall(r'"([a-z0-9-]+)"', m.group(1))


def census_defective(name: str) -> bool:
    """The 2026-09-06 shape: the guard's name near FAIL/violation anywhere in the Rust tree - INCLUDING guards.rs,
    whose registry lists every name a few hundred bytes from the words it is looking for."""
    for path in glob.glob(os.path.join(REPO, "keel-cli", "**", "*.rs"), recursive=True):
        text = open(path, encoding="utf-8", errors="replace").read()
        for m in re.finditer(re.escape(name), text):
            window = text[max(0, m.start() - 400): m.end() + 400]
            if "FAIL" in window or "violation" in window:
                return True
    return False


def census_corrected(name: str) -> bool:
    """A test in keel-cli/tests/ that runs `guard <name>` and asserts on FAIL in the same function body."""
    for path in glob.glob(os.path.join(REPO, "keel-cli", "tests", "*.rs")):
        text = open(path, encoding="utf-8", errors="replace").read()
        for body in re.split(r"#\[test\]", text)[1:]:
            if f'"{name}"' in body and ("FAIL" in body):
                return True
    return False


def probe_census() -> None:
    print("(1) guard-test census")
    names = guard_names()
    known_pos = "instruments-declared"   # keel-cli/tests/instruments_declared_catches_it.rs asserts FAIL
    known_neg = "doc-guard-count"        # no file under keel-cli/tests/ names it
    assert known_pos in names and known_neg in names
    # the defective check against its probe
    d_pos, d_neg = census_defective(known_pos), census_defective(known_neg)
    verdict("defective census on the known-positive", d_pos is True, f"{known_pos} -> {d_pos}")
    verdict("defective census on the known-NEGATIVE reads covered (the defect, reproduced)", d_neg is True,
            f"{known_neg} -> {d_neg}: the probe would have caught it before any number was stated")
    total_d = sum(census_defective(n) for n in names)
    verdict("defective census over the tree flatters it", total_d == len(names), f"{total_d} of {len(names)} - the 64-of-64 artefact")
    # the corrected check against the same probe
    c_pos, c_neg = census_corrected(known_pos), census_corrected(known_neg)
    verdict("corrected census on the known-positive", c_pos is True, f"{known_pos} -> {c_pos}")
    verdict("corrected census on the known-negative", c_neg is False, f"{known_neg} -> {c_neg}")
    total_c = sum(census_corrected(n) for n in names)
    print(f"        corrected census over the tree: {total_c} of {len(names)} (a figure, not a measure - see the not-an-instrument line)")


# ------------------------------------------------------------------ (2) the controller / anchor check
def _keel() -> str:
    # Resolve the binary CROSS-PLATFORM (D0394): the CI host is Linux with target/release/keel (no
    # .exe), while a Windows session runs keel-serve.exe / keel.exe. Try the built binary under each
    # name, then fall back to PATH (the CI script-probe step exports target/release onto it).
    import shutil
    rel = os.path.join(REPO, "target", "release")
    for name in ("keel-serve.exe", "keel.exe", "keel-serve", "keel"):
        p = os.path.join(rel, name)
        if os.path.exists(p):
            return p
    return shutil.which("keel") or "keel"


def control_structure() -> dict:
    out = subprocess.run([_keel(), "show", "control-structure", "."], capture_output=True, text=True, encoding="utf-8")
    if out.returncode != 0:
        raise SystemExit(f"issue400 reproduction: `keel show control-structure` failed ({out.returncode}): {out.stderr[-300:]}")
    return json.loads(out.stdout)


def anchor_defective(role: str, anchor: str) -> bool:
    """The 2026-09-07 shape: role name compared to anchor name as written - `human` vs `ctHuman`."""
    return role == anchor


def anchor_corrected(role: str, anchor: str) -> bool:
    """The anchor is `ct` + the role in CamelCase (`commit-gate` -> `ctCommitGate`)."""
    camel = "".join(p[:1].upper() + p[1:] for p in role.split("-"))
    return anchor == f"ct{camel}"


def probe_anchor() -> None:
    print("(2) controller anchor check")
    cs = control_structure()
    rows = {c["role"]: c for c in cs["controllers"]}
    known_pos = "human"            # ctHuman exists and carries a process model (control-structure view)
    assert rows[known_pos].get("processModel"), "the known-positive must hold before the probe is worth anything"
    r = rows[known_pos]
    d = anchor_defective(r["role"], r["anchor"])
    verdict("defective comparison on the known-positive reads ABSENT (the defect, reproduced)", d is False,
            f"{r['role']!r} == {r['anchor']!r} -> {d}: stated in a response before it was recomputed")
    missing_d = [c["role"] for c in cs["controllers"] if not anchor_defective(c["role"], c["anchor"])]
    verdict("defective comparison maligns every controller", len(missing_d) == len(cs["controllers"]), f"{len(missing_d)} of {len(cs['controllers'])} read as anchorless")
    c = anchor_corrected(r["role"], r["anchor"])
    verdict("corrected comparison on the known-positive", c is True, f"{r['role']!r} -> {r['anchor']!r}: {c}")
    known_neg = ("channel", "ctChannel")   # no controller of that role exists here (it sits in absentRoles)
    verdict("corrected comparison on a known-negative pair that is not this role's anchor",
            anchor_corrected("human", known_neg[1]) is False, f"human vs {known_neg[1]} -> False")
    missing_c = [c["role"] for c in cs["controllers"] if not anchor_corrected(c["role"], c["anchor"])]
    print(f"        corrected: {len(missing_c)} controller(s) whose anchor is not ct<Role>: {missing_c or 'none'}")


if __name__ == "__main__":
    # --probe (or no args) runs both reproductions as known-answer checks; the `# ci-probe:` marker at
    # the top is how the CI script-probe step discovers this file (D0394).
    if sys.argv[1:] not in ([], ["--probe"]):
        sys.exit("usage: python scripts/probes/issue400_reproduction.py [--probe]")
    probe_census()
    probe_anchor()
    print(f"issue400 reproduction: {FAILS} failure(s) - every FAIL above means a defect did NOT reproduce or a correction did not hold")
    sys.exit(1 if FAILS else 0)
