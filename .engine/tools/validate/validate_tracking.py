"""Validate the .tracking/ INSTANCE files against the pilot kernel. Loads the
engine artifact package first (so backlog's imports resolve), then each
.tracking/*.sysml. A file FAILS iff the kernel emits an error.

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
PRELOAD = [os.path.join(ENGINE, "workflows", "_meta.sysml")]   # EngineArtifacts (backlog imports it)
TRACKING = sorted(glob.glob(os.path.join(REPO, ".tracking", "*.sysml")))

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
