"""Validate the .tracking/ INSTANCE files against the pilot kernel. Preloads the
FULL schema (schema/core + safety + _meta) so instances may be typed by ANY engine
type — schema/core types are the canonical instance vocabulary (CR-1, 2026-06-11;
previously only _meta was preloaded, making every schema-typed instance fail).
Scans .tracking/ RECURSIVELY (subdirectories like business/ are sanctioned layout).
A file FAILS iff the kernel emits an error.

Run:  conda run -n sysml --no-capture-output python .engine/tools/validate/validate_tracking.py
"""
import os
import sys
import glob

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))  # .engine/tools
import _kernel  # noqa: E402

ENGINE = os.path.dirname(os.path.dirname(HERE))   # .engine
REPO = os.path.dirname(ENGINE)
PRELOAD = [os.path.join(ENGINE, *rel.split("/")) for rel in (
    "schema/core/element.sysml", "schema/core/needs.sysml", "schema/core/requirements.sysml",
    "schema/core/verification.sysml", "schema/core/work.sysml", "schema/core/architecture.sysml",
    "schema/core/computed.sysml", "schema/core/relationships.sysml", "schema/core/workflow.sysml",
    "schema/core/process.sysml", "schema/core/skills.sysml", "schema/core/risk.sysml",
    "schema/safety/stpa.sysml", "workflows/_meta.sysml",
)]
TRACKING = sorted(glob.glob(os.path.join(REPO, ".tracking", "**", "*.sysml"), recursive=True))

ERR = ("error", "couldn't", "cannot", "unexpected", "mismatched",
       "no viable", "unresolved", "extraneous", "wasn't expected")


def main():
    km, kc = _kernel.start()
    for f in PRELOAD:
        _kernel.run_cell(kc, open(f, encoding="utf-8").read())
    results = []
    for f in TRACKING:
        status, text = _kernel.run_cell(kc, open(f, encoding="utf-8").read())
        bad = any(w in (text or "").lower() for w in ERR)
        results.append((os.path.basename(f), not bad))
        print(f"[{'PASS' if not bad else 'FAIL'}] {os.path.basename(f)}")
        if bad:
            print("    " + (text or "").strip().replace("\n", "\n    ")[:600])
    print("=" * 48)
    print(f"  {sum(1 for _, p in results if p)}/{len(results)} tracking files passed")
    _kernel.teardown_and_exit(km, 0 if all(p for _, p in results) else 1)


if __name__ == "__main__":
    main()
