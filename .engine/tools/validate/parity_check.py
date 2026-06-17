r"""Parity / self-consistency gate for the Rust `sysmlv2` orient AUTHORITY (D0048).

KERNEL-FREE by design: post-D0048 the Rust toolchain is the canonical orient/validate
path, so the routine gate must not start the JVM (starting it was orphaning kernels —
the very leak Sprint 17 eliminates, issue004). This gate:

  * Runs `sysmlv2 orient .` (fast, no kernel).
  * Asserts the Rust done+outstanding total equals the structural number of distinct
    `action <name>;` task declarations in .tracking — catches the phantom-task /
    undercount class that motivated Sprint 17 (the bug that hid for ~10 sprints).
  * Sanity-checks the qualitative sets parse (ready/suspect/invalidEvidence lists).

The Python `query.py orient` cross-check (kernel-backed, secondary) is NOT run here —
it is a deliberate, occasional reconciliation. Run it by hand when reconciling:
    .\target\release\sysmlv2.exe orient .
    conda run -n sysml --no-capture-output python .engine\tools\query.py orient
and diff. query.py currently undercounts done by 1 (issue012); Rust is the oracle.

Run (from repo root):  python .engine/tools/validate/parity_check.py
Needs target/release/sysmlv2.exe built.
"""
import json
import os
import re
import subprocess
import sys
import glob

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
RUST_BIN = os.path.join(REPO, "target", "release", "sysmlv2.exe")


def rust_orient():
    if not os.path.exists(RUST_BIN):
        print(f"FAIL: {RUST_BIN} not built — run `cargo build --release`")
        sys.exit(1)
    out = subprocess.run([RUST_BIN, "orient", REPO], capture_output=True, text=True)
    m = re.search(r"\{.*\}", out.stdout, re.DOTALL)
    if not m:
        print("FAIL: could not parse sysmlv2 orient output:")
        print(out.stdout[:400] or out.stderr[:400])
        sys.exit(1)
    return json.loads(m.group(0))


def structural_task_count():
    text = "\n".join(
        open(f, encoding="utf-8", errors="replace").read()
        for f in glob.glob(os.path.join(REPO, ".tracking", "**", "*.sysml"), recursive=True)
    )
    return len(set(re.findall(r"\baction\s+(\w+)\s*;", text)))


def main():
    rust = rust_orient()
    structural = structural_task_count()
    done = rust["counts"]["done"]
    outstanding = rust["counts"]["outstanding"]
    total = done + outstanding
    in_prog = len(rust.get("in_progress_sprints", []))
    print(f"rust orient (AUTHORITY): done={done} outstanding={outstanding} total={total}")
    print(f"structural action-tasks: {structural}")
    print(f"in_progress_sprints: {in_prog}  ready: {len(rust.get('ready', []))}  "
          f"suspect: {len(rust.get('suspect', []))}  invalidEvidence: {len(rust.get('invalidEvidence', []))}")

    errors = 0
    if total != structural:
        print(f"  ERROR: orient total {total} != structural {structural} "
              f"(phantom-task / undercount regression)")
        errors += 1
    # in_progress pending must be a real canonical gate (or null)
    gates = {"Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"}
    for s in rust.get("in_progress_sprints", []):
        if s.get("pending") not in gates and s.get("pending") is not None:
            print(f"  ERROR: sprint {s.get('sprint')} has non-canonical pending {s.get('pending')!r}")
            errors += 1

    print(f"\n{'=' * 56}")
    if errors:
        print(f"FAIL — {errors} self-consistency violation(s)")
        sys.exit(1)
    print("PASS — rust authority self-consistent (total == structural task count)")


if __name__ == "__main__":
    main()
